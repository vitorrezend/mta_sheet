//! Renderizador Reativo de Texto Enriquecido / Markdown para o Compêndio.
//!
//! Converte marcações leves (parágrafos, negrito, itálico, cabeçalhos, listas
//! e hyperlinks `mta://` ou web) diretamente em elementos Leptos reativos e seguros.

use leptos::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineSpan {
    Text(String),
    Bold(String),
    Italic(String),
    Link {
        text: String,
        url: String,
        is_internal: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RichBlock {
    Heading { level: usize, text: String },
    RankHeading { dots: String, title: String },
    Paragraph(Vec<InlineSpan>),
    Quote(Vec<InlineSpan>),
    BulletList(Vec<Vec<InlineSpan>>),
    Divider,
}

/// Identifica se um cabeçalho representa um posto de Esfera (ex: "• Percepções Espaciais...").
fn parse_rank_heading(heading_text: &str) -> Option<(String, String)> {
    let trimmed = heading_text.trim();
    let mut dots_end = 0;
    let mut has_dots = false;

    for (idx, ch) in trimmed.char_indices() {
        if ch == '•' || ch == '●' || ch == '*' {
            has_dots = true;
            dots_end = idx + ch.len_utf8();
        } else if ch == ' ' && has_dots {
            dots_end = idx;
            break;
        } else {
            break;
        }
    }

    if has_dots && dots_end > 0 {
        let dots = trimmed[..dots_end].trim().to_string();
        let title = trimmed[dots_end..].trim().to_string();
        if !title.is_empty() {
            return Some((dots, title));
        }
    }
    None
}

/// Faz o parsing de spans inline (negrito, itálico e links).
pub fn parse_inline_spans(input: &str) -> Vec<InlineSpan> {
    let mut spans = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        // 1. Procura por Link [texto](url)
        if let Some(link_start) = remaining.find('[') {
            if let Some(link_mid) = remaining[link_start..].find("](") {
                let abs_mid = link_start + link_mid;
                if let Some(link_end) = remaining[abs_mid..].find(')') {
                    let abs_end = abs_mid + link_end;

                    // Texto antes do link
                    if link_start > 0 {
                        parse_styling(&remaining[..link_start], &mut spans);
                    }

                    let link_text = &remaining[link_start + 1..abs_mid];
                    let link_url = &remaining[abs_mid + 2..abs_end];
                    let is_internal = link_url.starts_with("mta://");

                    spans.push(InlineSpan::Link {
                        text: link_text.to_string(),
                        url: link_url.to_string(),
                        is_internal,
                    });

                    remaining = &remaining[abs_end + 1..];
                    continue;
                }
            }
        }

        // Não há mais links, processa o restante para negrito/itálico
        parse_styling(remaining, &mut spans);
        break;
    }

    spans
}

/// Processa estilos básicos (**negrito** e *itálico*).
fn parse_styling(text: &str, spans: &mut Vec<InlineSpan>) {
    let mut cur = text;
    while !cur.is_empty() {
        // Tenta achar **negrito**
        if let Some(bold_start) = cur.find("**") {
            if let Some(bold_end) = cur[bold_start + 2..].find("**") {
                let abs_end = bold_start + 2 + bold_end;
                if bold_start > 0 {
                    spans.push(InlineSpan::Text(cur[..bold_start].to_string()));
                }
                spans.push(InlineSpan::Bold(cur[bold_start + 2..abs_end].to_string()));
                cur = &cur[abs_end + 2..];
                continue;
            }
        }

        // Tenta achar *itálico*
        if let Some(ital_start) = cur.find('*') {
            if let Some(ital_end) = cur[ital_start + 1..].find('*') {
                let abs_end = ital_start + 1 + ital_end;
                if ital_start > 0 {
                    spans.push(InlineSpan::Text(cur[..ital_start].to_string()));
                }
                spans.push(InlineSpan::Italic(cur[ital_start + 1..abs_end].to_string()));
                cur = &cur[abs_end + 1..];
                continue;
            }
        }

        spans.push(InlineSpan::Text(cur.to_string()));
        break;
    }
}

/// Faz o parsing de um documento markdown completo em blocos estruturados.
pub fn parse_rich_document(text: &str) -> Vec<RichBlock> {
    let mut blocks = Vec::new();
    let mut current_para_lines: Vec<String> = Vec::new();
    let mut current_quote_lines: Vec<String> = Vec::new();
    let mut current_list_lines: Vec<String> = Vec::new();

    let flush_para = |lines: &mut Vec<String>, blocks: &mut Vec<RichBlock>| {
        if !lines.is_empty() {
            let joined = lines.join(" ");
            let trimmed = joined.trim();
            if !trimmed.is_empty() {
                blocks.push(RichBlock::Paragraph(parse_inline_spans(trimmed)));
            }
            lines.clear();
        }
    };

    let flush_quote = |lines: &mut Vec<String>, blocks: &mut Vec<RichBlock>| {
        if !lines.is_empty() {
            let joined = lines.join(" ");
            let trimmed = joined.trim();
            if !trimmed.is_empty() {
                blocks.push(RichBlock::Quote(parse_inline_spans(trimmed)));
            }
            lines.clear();
        }
    };

    let flush_list = |lines: &mut Vec<String>, blocks: &mut Vec<RichBlock>| {
        if !lines.is_empty() {
            let items = lines
                .drain(..)
                .map(|line| parse_inline_spans(&line))
                .collect();
            blocks.push(RichBlock::BulletList(items));
        }
    };

    for raw_line in text.lines() {
        let trimmed = raw_line.trim();

        // Linha vazia: finaliza os blocos acumulados
        if trimmed.is_empty() {
            flush_para(&mut current_para_lines, &mut blocks);
            flush_quote(&mut current_quote_lines, &mut blocks);
            flush_list(&mut current_list_lines, &mut blocks);
            continue;
        }

        // Divisor horizontal: --- ou *** ou ___
        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            flush_para(&mut current_para_lines, &mut blocks);
            flush_quote(&mut current_quote_lines, &mut blocks);
            flush_list(&mut current_list_lines, &mut blocks);
            blocks.push(RichBlock::Divider);
            continue;
        }

        // Citação em bloco (> ...)
        if let Some(quote_content) = trimmed.strip_prefix("> ") {
            flush_para(&mut current_para_lines, &mut blocks);
            flush_list(&mut current_list_lines, &mut blocks);
            current_quote_lines.push(quote_content.trim().to_string());
            continue;
        } else if trimmed == ">" {
            flush_para(&mut current_para_lines, &mut blocks);
            flush_list(&mut current_list_lines, &mut blocks);
            continue;
        } else {
            flush_quote(&mut current_quote_lines, &mut blocks);
        }

        // Lista de marcadores (- ou *)
        if let Some(item) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
            flush_para(&mut current_para_lines, &mut blocks);
            current_list_lines.push(item.trim().to_string());
            continue;
        } else {
            flush_list(&mut current_list_lines, &mut blocks);
        }

        // Cabeçalhos Markdown (#, ##, ###, ####, #####, ######)
        if trimmed.starts_with('#') {
            let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
            if hash_count > 0 && trimmed[hash_count..].starts_with(' ') {
                flush_para(&mut current_para_lines, &mut blocks);
                flush_quote(&mut current_quote_lines, &mut blocks);
                flush_list(&mut current_list_lines, &mut blocks);

                let heading_text = trimmed[hash_count..].trim();
                if let Some((dots, title)) = parse_rank_heading(heading_text) {
                    blocks.push(RichBlock::RankHeading { dots, title });
                } else {
                    blocks.push(RichBlock::Heading {
                        level: hash_count,
                        text: heading_text.to_string(),
                    });
                }
                continue;
            }
        }

        // Linha regular de parágrafo
        current_para_lines.push(trimmed.to_string());
    }

    flush_para(&mut current_para_lines, &mut blocks);
    flush_quote(&mut current_quote_lines, &mut blocks);
    flush_list(&mut current_list_lines, &mut blocks);

    blocks
}

/// Componente que renderiza um texto enriquecido em elementos HTML seguros.
#[component]
pub fn RichTextView(
    #[prop(into)] text: Signal<String>,
    #[prop(into, default = None)] class: Option<String>,
) -> impl IntoView {
    let parsed_blocks = Signal::derive(move || parse_rich_document(&text.get()));
    let extra_class = class.unwrap_or_default();

    view! {
        <div class=format!("compendium-rich-text {}", extra_class)>
            {move || {
                parsed_blocks.get().into_iter().map(|block| {
                    match block {
                        RichBlock::RankHeading { dots, title } => {
                            view! {
                                <div class="compendium-rank-header" style="margin-top: 1.4rem; margin-bottom: 0.5rem; display: flex; align-items: baseline; gap: 0.5rem; border-bottom: 1px solid var(--border-color, #e2e8f0); padding-bottom: 0.35rem;">
                                    <span class="compendium-rank-dots" style="font-weight: 800; font-size: 1.15rem; color: #4f46e5; letter-spacing: 0.08em; flex-shrink: 0;">
                                        {dots}
                                    </span>
                                    <strong class="compendium-rank-title" style="font-weight: 700; font-size: 0.98rem; color: var(--text-primary, #1e293b); line-height: 1.4;">
                                        {title}
                                    </strong>
                                </div>
                            }.into_view()
                        }
                        RichBlock::Heading { level, text } => {
                            let style = match level {
                                1 => "font-size: 1.3rem; font-weight: 800; margin-top: 1.4rem; margin-bottom: 0.5rem; color: var(--text-primary, #0f172a);",
                                2 => "font-size: 1.15rem; font-weight: 700; margin-top: 1.2rem; margin-bottom: 0.45rem; color: var(--text-primary, #1e293b);",
                                3 => "font-size: 1.05rem; font-weight: 700; margin-top: 1rem; margin-bottom: 0.4rem; color: var(--text-primary, #1e293b);",
                                _ => "font-size: 0.95rem; font-weight: 600; margin-top: 0.8rem; margin-bottom: 0.3rem; color: var(--text-primary, #334155);",
                            };
                            view! {
                                <h4 class=format!("compendium-rich-heading level-{}", level) style=style>
                                    {text}
                                </h4>
                            }.into_view()
                        }
                        RichBlock::Divider => {
                            view! {
                                <hr class="compendium-rich-divider" style="margin: 1.4rem 0; border: 0; border-top: 1px solid var(--border-color, #e2e8f0);" />
                            }.into_view()
                        }
                        RichBlock::Quote(spans) => {
                            view! {
                                <blockquote class="compendium-rich-quote" style="margin: 0.8rem 0 1.2rem 0; padding: 0.6rem 0.9rem; background: var(--surface-card, #f8fafc); border-left: 3px solid #6366f1; border-radius: 4px; font-size: 0.88rem; line-height: 1.5; color: var(--text-secondary, #475569);">
                                    {render_spans(spans)}
                                </blockquote>
                            }.into_view()
                        }
                        RichBlock::BulletList(items) => {
                            view! {
                                <ul class="compendium-rich-list">
                                    {items.into_iter().map(|item_spans| {
                                        view! {
                                            <li class="compendium-rich-list-item">
                                                {render_spans(item_spans)}
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            }.into_view()
                        }
                        RichBlock::Paragraph(spans) => {
                            view! {
                                <p class="compendium-rich-paragraph" style="margin: 0 0 0.95rem 0; line-height: 1.62; font-size: 0.92rem; color: var(--text-secondary, #334155);">
                                    {render_spans(spans)}
                                </p>
                            }.into_view()
                        }
                    }
                }).collect_view()
            }}
        </div>
    }
}

fn render_spans(spans: Vec<InlineSpan>) -> impl IntoView {
    spans.into_iter().map(|span| {
        match span {
            InlineSpan::Text(t) => view! { <span>{t}</span> }.into_view(),
            InlineSpan::Bold(b) => view! { <strong>{b}</strong> }.into_view(),
            InlineSpan::Italic(i) => view! { <em>{i}</em> }.into_view(),
            InlineSpan::Link { text, url, is_internal } => {
                if is_internal {
                    view! {
                        <a
                            href=url
                            class="compendium-internal-link"
                            title="Navegar no Compêndio"
                        >
                            {text}
                            <span class="link-arrow">" ↗"</span>
                        </a>
                    }.into_view()
                } else {
                    view! {
                        <a
                            href=url
                            target="_blank"
                            rel="noopener noreferrer"
                            class="compendium-external-link"
                        >
                            {text}
                        </a>
                    }.into_view()
                }
            }
        }
    }).collect_view()
}
