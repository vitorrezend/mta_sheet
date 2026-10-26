//! Menu "⋯" da ficha, em seções: Personagem, Compartilhar, Arquivo e Exibição.

use leptos::*;
use crate::components::Callback;
use crate::i18n::Language;
use super::super::SheetLayout;
use super::{pick, SheetTopBarActions};

/// Item que abre algo (janela, arquivo, página). Fecha o menu ao clicar.
#[component]
fn MenuItem(
    icon: &'static str,
    title: Signal<&'static str>,
    hint: Signal<&'static str>,
    #[prop(optional)] extra_class: &'static str,
    on_select: Callback<()>,
) -> impl IntoView {
    view! {
        <button type="button" class=format!("actions-menu-item-btn {}", extra_class) on:click=move |_| on_select.call(())>
            <span class="action-icon">{icon}</span>
            <span class="action-text">
                <span class="action-label">{move || title.get()}</span>
                <span class="action-hint">{move || hint.get()}</span>
            </span>
        </button>
    }
}

/// Linha com interruptor (não fecha o menu: o efeito aparece na hora).
#[component]
fn ToggleRow(
    icon: Signal<&'static str>,
    title: Signal<&'static str>,
    hint: Signal<&'static str>,
    on: Signal<bool>,
    #[prop(optional)] extra_class: &'static str,
    on_toggle: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            role="switch"
            class=format!("menu-toggle-row {}", extra_class)
            aria-checked=move || if on.get() { "true" } else { "false" }
            on:click=move |_| on_toggle.call(())
        >
            <span class="action-icon">{move || icon.get()}</span>
            <span class="action-text">
                <span class="action-label">{move || title.get()}</span>
                <span class="action-hint">{move || hint.get()}</span>
            </span>
            <span class="menu-switch" class:on=move || on.get() aria-hidden="true"><span class="menu-switch-knob"></span></span>
        </button>
    }
}

#[component]
pub fn MoreMenu(
    open: RwSignal<bool>,
    can_edit: MaybeSignal<bool>,
    is_public: Signal<bool>,
    layout: RwSignal<SheetLayout>,
    lang: Signal<Language>,
    actions: SheetTopBarActions,
    /// Abre o seletor de arquivo para importar JSON
    on_pick_import: Callback<()>,
) -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let t = move |pt: &'static str, en: &'static str| Signal::derive(move || pick(lang.get(), pt, en));
    // Cada item fecha o menu e depois executa a ação
    let then_close = move |f: Callback<()>| Callback::new(move |_| {
        open.set(false);
        f.call(());
    });
    let SheetTopBarActions {
        on_toggle_privacy,
        on_export_json,
        set_show_breakdown,
        set_show_quiz,
        set_show_pdf_modal,
        set_show_share_modal,
        ..
    } = actions;

    let open_pdf = Callback::new(move |_| match set_show_pdf_modal {
        Some(set) => set.set(true),
        None => {
            if let Some(w) = web_sys::window() {
                let _ = w.print();
            }
        }
    });
    let open_share = Callback::new(move |_| {
        if let Some(set) = set_show_share_modal {
            set.set(true);
        }
    });

    view! {
        <div class="actions-menu-backdrop" class:open=move || open.get() on:click=move |_| open.set(false)>
            <div class="actions-menu-panel" role="menu" on:click=move |ev| ev.stop_propagation()>
                <div class="actions-menu-panel-header">
                    <span class="panel-title">{move || pick(lang.get(), "Opções da ficha", "Sheet options")}</span>
                    <button type="button" class="panel-close-btn" aria-label="Fechar" on:click=move |_| open.set(false)>"✕"</button>
                </div>
                <div class="actions-menu-panel-body">
                    <div class="menu-section-title">{move || pick(lang.get(), "Personagem", "Character")}</div>
                    <MenuItem
                        icon="📂"
                        title=t("Dossiê do personagem", "Character dossier")
                        hint=t("Questionário de criação e anotações de fundo", "Creation questionnaire and background notes")
                        extra_class="dossier-menu-btn"
                        on_select=then_close(Callback::new(move |_| set_show_quiz.set(true)))
                    />
                    <MenuItem
                        icon="📊"
                        title=t("Extrato de custos e XP", "Cost and XP statement")
                        hint=t("Pontos gastos em cada modo e registro de experiência", "Points spent per mode and experience log")
                        on_select=then_close(Callback::new(move |_| set_show_breakdown.set(true)))
                    />

                    <Show when=move || can_edit.get()>
                        <div class="menu-section-title">{move || pick(lang.get(), "Compartilhar", "Sharing")}</div>
                        <MenuItem
                            icon="🔗"
                            title=t("Compartilhar ficha…", "Share sheet…")
                            hint=t("Pessoas, salas de crônica e link público", "People, chronicle rooms and public link")
                            extra_class="share-menu-item-btn"
                            on_select=then_close(open_share.clone())
                        />
                        <ToggleRow
                            icon=Signal::derive(move || if is_public.get() { "🌐" } else { "🔒" })
                            title=t("Ficha pública", "Public sheet")
                            hint=Signal::derive(move || match (is_public.get(), lang.get()) {
                                (true, Language::PtBr) => "Aparece na aba de fichas públicas e pode ser vista por todos",
                                (false, Language::PtBr) => "Só você e quem recebeu acesso podem ver",
                                (true, Language::EnUs) => "Listed under public sheets and visible to everyone",
                                (false, Language::EnUs) => "Only you and people you shared it with can see it",
                            })
                            on=is_public
                            extra_class="privacy-toggle-row"
                            on_toggle=on_toggle_privacy.clone()
                        />
                    </Show>

                    <div class="menu-section-title">{move || pick(lang.get(), "Arquivo", "File")}</div>
                    <Show when=move || can_edit.get()>
                        <MenuItem
                            icon="📥"
                            title=t("Importar ficha (.json)", "Import sheet (.json)")
                            hint=t("Substitui o conteúdo desta ficha pelo arquivo", "Replaces this sheet's content with the file")
                            extra_class="json-import-btn"
                            on_select=then_close(on_pick_import.clone())
                        />
                    </Show>
                    <MenuItem
                        icon="📤"
                        title=t("Exportar ficha (.json)", "Export sheet (.json)")
                        hint=t("Cópia de segurança para guardar ou importar depois", "Backup copy to keep or import later")
                        extra_class="json-export-btn"
                        on_select=then_close(on_export_json.clone())
                    />
                    <MenuItem
                        icon="🖨️"
                        title=t("Exportar em PDF (A4)", "Export as PDF (A4)")
                        hint=t("Páginas da ficha prontas para imprimir", "Sheet pages ready to print")
                        extra_class="export-pdf-btn"
                        on_select=then_close(open_pdf)
                    />

                    <div class="menu-section-title">{move || pick(lang.get(), "Exibição", "Display")}</div>
                    <ToggleRow
                        icon=Signal::derive(|| "🌐")
                        title=t("Interface em inglês", "English interface")
                        hint=Signal::derive(move || match lang.get() {
                            Language::PtBr => "Idioma atual: Português",
                            Language::EnUs => "Current language: English",
                        })
                        on=Signal::derive(move || lang.get() == Language::EnUs)
                        extra_class="lang-toggle-row"
                        on_toggle=Callback::new(move |_| {
                            if let Some(ctx) = lang_ctx {
                                ctx.toggle();
                            }
                        })
                    />
                    // Só em telas largas (CSS): no celular e na impressão a ficha é sempre A4
                    <ToggleRow
                        icon=Signal::derive(|| "↔️")
                        title=t("Ficha larga", "Wide sheet")
                        hint=Signal::derive(move || match (layout.get().is_wide(), lang.get()) {
                            (true, Language::PtBr) => "Usa mais espaço da tela; PDF e celular continuam em A4",
                            (false, Language::PtBr) => "Atual: folha A4",
                            (true, Language::EnUs) => "Uses more screen space; PDF and phones stay A4",
                            (false, Language::EnUs) => "Current: A4 page",
                        })
                        on=Signal::derive(move || layout.get().is_wide())
                        extra_class="layout-toggle-btn"
                        on_toggle=Callback::new(move |_| layout.update(|l| *l = l.toggled()))
                    />

                    <a href="/logs" class="actions-menu-item-btn menu-logs-link" on:click=move |_| open.set(false)>
                        <span class="action-icon">"🧾"</span>
                        <span class="action-text">
                            <span class="action-label">{move || pick(lang.get(), "Logs do sistema", "System logs")}</span>
                        </span>
                    </a>
                </div>
            </div>
        </div>
    }
}
