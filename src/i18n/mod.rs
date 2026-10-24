use leptos::*;

pub mod traits;
pub mod dossier;
pub mod auth;
pub mod ui;

pub use traits::*;
pub use dossier::*;
pub use auth::*;
pub use ui::*;

pub use mta_core::i18n::Language;

pub fn detect_browser_language() -> Language {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            // 1. Check user preference stored in localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved)) = storage.get_item("mta_sheet_lang") {
                    if saved.eq_ignore_ascii_case("en") {
                        return Language::EnUs;
                    } else if saved.eq_ignore_ascii_case("pt") {
                        return Language::PtBr;
                    }
                }
            }
            // 2. Check browser navigator language
            let nav = window.navigator();
            if let Some(lang_str) = nav.language() {
                if lang_str.to_lowercase().starts_with("pt") {
                    return Language::PtBr;
                } else {
                    return Language::EnUs;
                }
            }
        }
    }
    Language::PtBr
}

#[allow(unused_variables)]
pub fn save_language_preference(lang: Language) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("mta_sheet_lang", lang.code());
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct LanguageContext {
    pub lang: ReadSignal<Language>,
    pub set_lang: WriteSignal<Language>,
}

impl LanguageContext {
    pub fn new(lang: ReadSignal<Language>, set_lang: WriteSignal<Language>) -> Self {
        Self { lang, set_lang }
    }

    pub fn toggle(&self) {
        let current = self.lang.get();
        let next = match current {
            Language::PtBr => Language::EnUs,
            Language::EnUs => Language::PtBr,
        };
        self.set_lang.set(next);
        save_language_preference(next);
    }
}
