use serde::Serialize;
use crate::i18n::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CalloutBox {
    pub title: &'static str,
    pub title_pt: &'static str,
    pub page_ref: &'static str,
    pub content: &'static str,
    pub content_pt: &'static str,
}

impl CalloutBox {
    pub fn title(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.title_pt,
            Language::EnUs => self.title,
        }
    }

    pub fn content(&self, lang: Language) -> &'static str {
        match lang {
            Language::PtBr => self.content_pt,
            Language::EnUs => self.content,
        }
    }
}

pub const BOX_LEFT_AND_RIGHT_HAND_PATHS: CalloutBox = CalloutBox {
    title: "Left- and Right-Hand Paths",
    title_pt: "Caminhos da Mão Esquerda e da Mão Direita",
    page_ref: "M20, p. 574",
    content: include_str!("../../../data/compendium/practices/left_and_right_hand_paths.en.md"),
    content_pt: include_str!("../../../data/compendium/practices/left_and_right_hand_paths.pt.md"),
};
