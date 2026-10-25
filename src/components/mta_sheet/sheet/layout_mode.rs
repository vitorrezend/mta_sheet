//! Largura da ficha na tela: folha A4 (padrão) ou "Larga", escolhida por quem usa e
//! guardada no navegador. Celular e impressão ficam sempre em A4 (regras no CSS).

use leptos::*;

const STORAGE_KEY: &str = "mta_sheet_layout";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetLayout {
    #[default]
    A4,
    Wide,
}

impl SheetLayout {
    fn storage_value(self) -> &'static str {
        match self {
            Self::A4 => "a4",
            Self::Wide => "wide",
        }
    }

    /// Valor salvo no navegador; qualquer coisa desconhecida volta ao A4.
    pub fn from_storage(value: &str) -> Self {
        if value == "wide" { Self::Wide } else { Self::A4 }
    }

    pub fn toggled(self) -> Self {
        match self {
            Self::A4 => Self::Wide,
            Self::Wide => Self::A4,
        }
    }

    pub fn is_wide(self) -> bool {
        self == Self::Wide
    }
}

fn load() -> Option<SheetLayout> {
    #[cfg(target_arch = "wasm32")]
    {
        let storage = web_sys::window().and_then(|w| w.local_storage().ok()).flatten()?;
        return storage.get_item(STORAGE_KEY).ok().flatten().map(|v| SheetLayout::from_storage(&v));
    }
    #[cfg(not(target_arch = "wasm32"))]
    None
}

fn save(layout: SheetLayout) {
    #[cfg(target_arch = "wasm32")]
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
        let _ = storage.set_item(STORAGE_KEY, layout.storage_value());
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (layout.storage_value(), STORAGE_KEY);
}

/// Sinal da largura da ficha, ligado ao navegador.
///
/// Começa em A4, como o HTML vindo do servidor (que não conhece a preferência), e só
/// depois da hidratação adota o valor salvo: aplicar antes quebraria a hidratação.
pub fn use_sheet_layout() -> RwSignal<SheetLayout> {
    let layout = create_rw_signal(SheetLayout::A4);
    // Efeitos só rodam no navegador, depois da hidratação. Este não lê sinais: roda uma vez.
    create_effect(move |_| {
        if let Some(saved) = load() {
            layout.set(saved);
        }
    });
    // Grava cada troca (a primeira execução só registra o valor inicial)
    create_effect(move |previous: Option<SheetLayout>| {
        let current = layout.get();
        if previous.is_some_and(|p| p != current) {
            save(current);
        }
        current
    });
    layout
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_saved_values_fall_back_to_a4() {
        assert_eq!(SheetLayout::from_storage("wide"), SheetLayout::Wide);
        assert_eq!(SheetLayout::from_storage("a4"), SheetLayout::A4);
        assert_eq!(SheetLayout::from_storage("enorme"), SheetLayout::A4);
    }

    #[test]
    fn toggling_alternates_between_the_two_layouts() {
        assert_eq!(SheetLayout::A4.toggled(), SheetLayout::Wide);
        assert_eq!(SheetLayout::Wide.toggled(), SheetLayout::A4);
    }
}
