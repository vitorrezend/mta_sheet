//! Título e descrição da página da ficha (aba do navegador, buscadores e prévias de link).

use leptos::*;
use crate::state::CharacterData;

fn tradition_or_default(d: &CharacterData) -> String {
    let tradition = d.get_tradition();
    let tradition = tradition.trim();
    if tradition.is_empty() { "Mago".to_string() } else { tradition.to_string() }
}

fn short_description(d: &CharacterData) -> String {
    format!(
        "Ficha de personagem {} ({}) para Mago: A Ascensão (M20) no MTA Sheet.",
        d.get_display_name(),
        tradition_or_default(d)
    )
}

fn description(d: &CharacterData) -> String {
    let concept = d.get_label("Concept");
    let concept = concept.trim();
    if concept.is_empty() {
        short_description(d)
    } else {
        format!(
            "Ficha de personagem {} ({}) - Conceito: {}. Sistema Mago: A Ascensão (M20).",
            d.get_display_name(),
            tradition_or_default(d),
            concept
        )
    }
}

#[component]
pub fn SheetMeta(data: ReadSignal<CharacterData>) -> impl IntoView {
    let title = move || format!("{} | MTA Sheet", data.with(|d| d.get_display_name()));
    view! {
        <leptos_meta::Title text=title />
        <leptos_meta::Meta name="description" content=move || data.with(description) />
        // Fichas privadas não devem aparecer em buscadores
        <leptos_meta::Meta
            name="robots"
            content=move || if data.with(|d| d.is_public) { "index, follow" } else { "noindex, nofollow" }
        />
        <leptos_meta::Meta property="og:title" content=title />
        <leptos_meta::Meta property="og:description" content=move || data.with(short_description) />
    }
}
