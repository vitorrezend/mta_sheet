use leptos::*;
use leptos::html::Div;
use wasm_bindgen::JsCast;
use super::callback::Callback;
use crate::i18n::Language;

/// Converte Markdown legado para HTML estruturado caso o texto ainda não possua tags HTML.
pub fn legacy_markdown_to_html(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    // Se já contiver tags HTML comuns, assume que já é HTML rico e apenas sanitiza
    if trimmed.contains("<p") 
        || trimmed.contains("<br") 
        || trimmed.contains("<div") 
        || trimmed.contains("<ul") 
        || trimmed.contains("<ol") 
        || trimmed.contains("<strong") 
        || trimmed.contains("<b") 
        || trimmed.contains("<em") 
        || trimmed.contains("<i") 
        || trimmed.contains("<u") 
        || trimmed.contains("<span") 
    {
        return sanitize_wysiwyg_html(trimmed);
    }

    let mut html = String::new();
    let mut in_ul = false;
    let mut in_ol = false;

    for line in input.lines() {
        let l_trim = line.trim();
        if l_trim.is_empty() {
            if in_ul {
                html.push_str("</ul>");
                in_ul = false;
            }
            if in_ol {
                html.push_str("</ol>");
                in_ol = false;
            }
            continue;
        }

        let is_bullet = l_trim.starts_with("- ") || l_trim.starts_with("* ") || l_trim.starts_with("• ");
        let is_num = l_trim.chars().next().map_or(false, |c| c.is_ascii_digit()) 
            && l_trim.chars().nth(1).map_or(false, |c| c == '.' || (c.is_ascii_digit() && l_trim.chars().nth(2) == Some('.')));

        if is_bullet {
            if in_ol {
                html.push_str("</ol>");
                in_ol = false;
            }
            if !in_ul {
                html.push_str("<ul>");
                in_ul = true;
            }
            let content = if l_trim.starts_with("• ") {
                &l_trim[3..] // "• " é 3 bytes em UTF-8
            } else {
                &l_trim[2..]
            };
            html.push_str("<li>");
            html.push_str(&inline_markdown_to_html(content));
            html.push_str("</li>");
        } else if is_num {
            if in_ul {
                html.push_str("</ul>");
                in_ul = false;
            }
            if !in_ol {
                html.push_str("<ol>");
                in_ol = true;
            }
            let dot_pos = l_trim.find('.').unwrap_or(1);
            let content = l_trim[dot_pos + 1..].trim_start();
            html.push_str("<li>");
            html.push_str(&inline_markdown_to_html(content));
            html.push_str("</li>");
        } else {
            if in_ul {
                html.push_str("</ul>");
                in_ul = false;
            }
            if in_ol {
                html.push_str("</ol>");
                in_ol = false;
            }
            html.push_str("<p>");
            html.push_str(&inline_markdown_to_html(l_trim));
            html.push_str("</p>");
        }
    }

    if in_ul {
        html.push_str("</ul>");
    }
    if in_ol {
        html.push_str("</ol>");
    }

    sanitize_wysiwyg_html(&html)
}

/// Converte formatação inline básica de markdown (**negrito**, *itálico*) para tags HTML seguras.
pub fn inline_markdown_to_html(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '*' {
            if chars.peek() == Some(&'*') {
                chars.next(); // consome segundo '*'
                let mut bold_content = String::new();
                let mut closed = false;
                while let Some(b) = chars.next() {
                    if b == '*' && chars.peek() == Some(&'*') {
                        chars.next();
                        closed = true;
                        break;
                    } else {
                        bold_content.push(b);
                    }
                }
                if closed {
                    out.push_str("<strong>");
                    out.push_str(&html_escape(&bold_content));
                    out.push_str("</strong>");
                } else {
                    out.push_str("**");
                    out.push_str(&html_escape(&bold_content));
                }
            } else {
                let mut ital_content = String::new();
                let mut closed = false;
                while let Some(i) = chars.next() {
                    if i == '*' {
                        closed = true;
                        break;
                    } else {
                        ital_content.push(i);
                    }
                }
                if closed {
                    out.push_str("<em>");
                    out.push_str(&html_escape(&ital_content));
                    out.push_str("</em>");
                } else {
                    out.push('*');
                    out.push_str(&html_escape(&ital_content));
                }
            }
        } else if c == '<' {
            out.push_str("&lt;");
        } else if c == '>' {
            out.push_str("&gt;");
        } else if c == '&' {
            out.push_str("&amp;");
        } else {
            out.push(c);
        }
    }

    out
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}

/// Sanitizador seguro para o conteúdo HTML gerado pelo editor WYSIWYG.
pub fn sanitize_wysiwyg_html(html: &str) -> String {
    let lower = html.to_lowercase();
    if lower.contains("<script") 
        || lower.contains("<iframe") 
        || lower.contains("<object") 
        || lower.contains("<embed") 
        || lower.contains("javascript:") 
        || lower.contains("onerror=") 
        || lower.contains("onload=") 
        || lower.contains("onclick=") 
    {
        html.replace("<script", "&lt;script")
            .replace("</script>", "&lt;/script&gt;")
            .replace("<iframe", "&lt;iframe")
            .replace("javascript:", "blocked:")
            .replace("onerror=", "data-err=")
            .replace("onload=", "data-load=")
            .replace("onclick=", "data-click=")
    } else {
        html.to_string()
    }
}

/// Executa um comando de edição nativo do navegador no documento ativo.
pub fn exec_browser_command(cmd: &str, val: Option<&str>) {
    #[cfg(target_arch = "wasm32")]
    {
        let js = match val {
            Some(v) => format!("document.execCommand('{}', false, '{}')", cmd, v.replace('\'', "\\'")),
            None => format!("document.execCommand('{}', false, null)", cmd),
        };
        let _ = js_sys::eval(&js);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (cmd, val);
    }
}

/// Componente Canônico Reutilizável de Editor WYSIWYG Real.
/// Fornece edição rica direta com Focus-Lock (previne engasgos de frame e perda de cursor),
/// atalhos de teclado (Tab/Esc/Ctrl+B/Ctrl+I/Ctrl+U), migração transparente de Markdown legado,
/// modo compacto para caixas menores e sanitização de segurança.
#[component]
pub fn WysiwygEditor(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into, optional)] placeholder: MaybeSignal<String>,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] min_height: Option<&'static str>,
    #[prop(default = false)] compact: bool,
    #[prop(into, optional)] lang: Option<Signal<Language>>,
) -> impl IntoView {
    let editor_ref = create_node_ref::<Div>();
    let is_focused = std::rc::Rc::new(std::cell::Cell::new(false));
    let last_synced = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let is_focused_focus = is_focused.clone();
    let is_focused_blur = is_focused.clone();
    let last_synced_blur = last_synced.clone();
    let is_focused_effect = is_focused.clone();
    let last_synced_effect = last_synced.clone();

    // Sincronização inicial e reativa protegida por Focus-Lock
    create_render_effect(move |_| {
        let raw = value.try_get().unwrap_or_default();
        let html_content = legacy_markdown_to_html(&raw);
        if !is_focused_effect.get() {
            if let Some(elem) = editor_ref.get_untracked() {
                let raw_elem: &web_sys::HtmlDivElement = &elem;
                let current_html = raw_elem.inner_html();
                if current_html != html_content {
                    raw_elem.set_inner_html(&html_content);
                }
            }
            *last_synced_effect.borrow_mut() = html_content;
        }
    });

    let exec_cmd = {
        let on_change = on_change.clone();
        let last_synced = last_synced.clone();
        let editor_ref = editor_ref;
        move |cmd: &'static str, val: Option<&'static str>| {
            if let Some(elem) = editor_ref.get_untracked() {
                let _ = elem.focus();
                exec_browser_command(cmd, val);
                let raw_elem: &web_sys::HtmlDivElement = &elem;
                let new_html = raw_elem.inner_html();
                *last_synced.borrow_mut() = new_html.clone();
                on_change.call(new_html);
            }
        }
    };

    let exec_bold = {
        let exec = exec_cmd.clone();
        move |_| exec("bold", None)
    };

    let exec_italic = {
        let exec = exec_cmd.clone();
        move |_| exec("italic", None)
    };

    let exec_underline = {
        let exec = exec_cmd.clone();
        move |_| exec("underline", None)
    };

    let exec_ul = {
        let exec = exec_cmd.clone();
        move |_| exec("insertUnorderedList", None)
    };

    let exec_ol = {
        let exec = exec_cmd.clone();
        move |_| exec("insertOrderedList", None)
    };

    let exec_clear = {
        let exec = exec_cmd;
        move |_| exec("removeFormat", None)
    };

    let active_lang = Signal::derive(move || {
        if let Some(l) = lang {
            l.get()
        } else if let Some(ctx) = use_context::<crate::i18n::LanguageContext>() {
            ctx.lang.get()
        } else {
            crate::i18n::Language::PtBr
        }
    });

    let last_synced_input = last_synced.clone();
    let on_change_tab = on_change.clone();
    let last_synced_tab = last_synced.clone();

    // Impede que o clique na barra de ferramentas desloque ou cancele a seleção ativa de texto
    let prevent_blur = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
    };

    let initial_html = {
        let raw = value.get_untracked();
        legacy_markdown_to_html(&raw)
    };

    let min_h_style = min_height.unwrap_or(if compact { "65px" } else { "95px" });
    let editor_style = format!("min-height: {};", min_h_style);

    view! {
        <div class=format!("wysiwyg-wrapper {}", class) class:wysiwyg-compact=compact>
            <div class="wysiwyg-toolbar" class:compact=compact>
                <button
                    type="button"
                    class="wysiwyg-btn"
                    title=move || match active_lang.get() {
                        Language::PtBr => "Negrito (Ctrl+B)",
                        Language::EnUs => "Bold (Ctrl+B)",
                    }
                    on:mousedown=prevent_blur
                    on:click=exec_bold
                >
                    <strong>"B"</strong>
                </button>
                <button
                    type="button"
                    class="wysiwyg-btn"
                    title=move || match active_lang.get() {
                        Language::PtBr => "Itálico (Ctrl+I)",
                        Language::EnUs => "Italic (Ctrl+I)",
                    }
                    on:mousedown=prevent_blur
                    on:click=exec_italic
                >
                    <em>"I"</em>
                </button>
                <button
                    type="button"
                    class="wysiwyg-btn"
                    title=move || match active_lang.get() {
                        Language::PtBr => "Sublinhado (Ctrl+U)",
                        Language::EnUs => "Underline (Ctrl+U)",
                    }
                    on:mousedown=prevent_blur
                    on:click=exec_underline
                >
                    <span style="text-decoration: underline;">"U"</span>
                </button>
                <button
                    type="button"
                    class="wysiwyg-btn"
                    title=move || match active_lang.get() {
                        Language::PtBr => "Lista de Marcadores",
                        Language::EnUs => "Bullet List",
                    }
                    on:mousedown=prevent_blur
                    on:click=exec_ul
                >
                    "• " {move || if compact { "" } else { match active_lang.get() {
                        Language::PtBr => "Lista",
                        Language::EnUs => "List",
                    } }}
                </button>
                <button
                    type="button"
                    class="wysiwyg-btn"
                    title=move || match active_lang.get() {
                        Language::PtBr => "Lista Numerada",
                        Language::EnUs => "Numbered List",
                    }
                    on:mousedown=prevent_blur
                    on:click=exec_ol
                >
                    "1. " {move || if compact { "" } else { match active_lang.get() {
                        Language::PtBr => "Números",
                        Language::EnUs => "Numbers",
                    } }}
                </button>
                <button
                    type="button"
                    class="wysiwyg-btn clear-btn"
                    title=move || match active_lang.get() {
                        Language::PtBr => "Limpar Formatação da Seleção",
                        Language::EnUs => "Clear Formatting",
                    }
                    on:mousedown=prevent_blur
                    on:click=exec_clear
                >
                    "🧹"
                </button>
            </div>

            <div
                node_ref=editor_ref
                class="wysiwyg-content"
                contenteditable="true"
                data-placeholder=move || placeholder.get()
                prop:innerHTML=initial_html
                style=editor_style
                on:focus=move |_| { is_focused_focus.set(true); }
                on:blur=move |ev: web_sys::FocusEvent| {
                    is_focused_blur.set(false);
                    if let Some(target) = ev.current_target() {
                        if let Ok(raw_elem) = target.dyn_into::<web_sys::HtmlDivElement>() {
                            let html_val = raw_elem.inner_html();
                            if html_val != *last_synced_blur.borrow() {
                                *last_synced_blur.borrow_mut() = html_val.clone();
                                on_change.call(html_val);
                            }
                        }
                    }
                }
                on:input=move |ev: web_sys::Event| {
                    // Atualiza apenas a cópia interna para evitar loops e reconstrução do DOM enquanto digita
                    if let Some(target) = ev.current_target() {
                        if let Ok(raw_elem) = target.dyn_into::<web_sys::HtmlDivElement>() {
                            *last_synced_input.borrow_mut() = raw_elem.inner_html();
                        }
                    }
                }
                on:keydown=move |ev: web_sys::KeyboardEvent| {
                    if ev.key() == "Tab" && !ev.shift_key() && !ev.ctrl_key() && !ev.alt_key() {
                        ev.prevent_default();
                        exec_browser_command("insertText", Some("  "));
                        if let Some(target) = ev.current_target() {
                            if let Ok(raw_elem) = target.dyn_into::<web_sys::HtmlDivElement>() {
                                let html_val = raw_elem.inner_html();
                                *last_synced_tab.borrow_mut() = html_val.clone();
                                on_change_tab.call(html_val);
                            }
                        }
                    } else if ev.key() == "Escape" {
                        if let Some(target) = ev.current_target() {
                            if let Ok(raw_elem) = target.dyn_into::<web_sys::HtmlDivElement>() {
                                let _ = raw_elem.blur();
                            }
                        }
                    }
                }
            />
        </div>
    }
}
