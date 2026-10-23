//! Linha de característica da ficha: nome, modificador, bolinhas de nível e escala sobrenatural.
//!
//! As partes ficam em arquivos próprios e recebem o mesmo [`TraitField`], que reúne
//! os dados e as ações compartilhados por elas.

mod dots;
mod label;
mod modifier;
mod summary;
mod supernatural;

use leptos::*;
use crate::state::DotOrigin;
use super::callback::Callback;
use dots::ValueDots;
use label::{AffinityStar, EditableName, FlipButton, NameTooltip, StaticName, SupernaturalMenu};
use modifier::ModifierCell;
use supernatural::SupernaturalSlot;

/// Dados e ações de um campo, compartilhados pelas partes dele.
#[derive(Clone)]
pub struct TraitField {
    pub label: Signal<String>,
    pub level: Signal<i32>,
    pub modifier: Signal<String>,
    /// Origem de cada ponto (Base, Bônus, XP, Buff); sem isso todos contam como Base
    pub origins: Option<Signal<Vec<DotOrigin>>>,
    pub suggested_specialties: Option<Signal<Vec<&'static str>>>,
    /// Página do livro mostrada no botão do compêndio
    pub page_ref_str: Signal<String>,
    /// Campo do modificador e o último valor digitado nele (focus-lock compartilhado com o painel sobrenatural)
    pub modifier_ref: NodeRef<html::Input>,
    pub last_modifier_value: RwSignal<String>,
    /// Bolinha com o menu de origem aberto
    pub popover: RwSignal<Option<usize>>,
    pub on_level_change: Callback<i32>,
    pub on_modifier_change: Callback<String>,
    pub on_origin_change: Option<Callback<(usize, DotOrigin)>>,
    pub on_open_compendium: Option<Callback<()>>,
}

/// Memoriza um sinal: quem lê só é avisado quando o valor muda de fato.
fn memo<T: Clone + PartialEq + 'static>(signal: Signal<T>) -> Signal<T> {
    create_memo(move |_| signal.get()).into()
}

/// Lê as origens dos pontos (quando o campo tem) sem clonar a lista.
fn with_origins<R>(origins: Option<Signal<Vec<DotOrigin>>>, f: impl FnOnce(Option<&[DotOrigin]>) -> R) -> R {
    match origins {
        Some(sig) => sig.with(|o| f(Some(o))),
        None => f(None),
    }
}

#[component]
pub fn ValueField(
    label: Signal<String>,
    level: Signal<i32>,
    modifier: Signal<String>,
    on_level_change: impl Fn(i32) + 'static,
    on_modifier_change: impl Fn(String) + 'static,
    #[prop(default = 0)] min_level: i32,
    #[prop(default = 5)] max_level: i32,
    #[prop(optional)] max_chars: Option<usize>,
    #[prop(optional)] on_remove: Option<Callback<()>>,
    #[prop(default = false)] is_editable: bool,
    #[prop(optional)] on_label_change: Option<Callback<String>>,
    #[prop(optional)] origins: Option<Signal<Vec<DotOrigin>>>,
    #[prop(optional)] on_dot_origin_change: Option<Callback<(usize, DotOrigin)>>,
    #[prop(optional)] is_starred: Option<Signal<bool>>,
    #[prop(optional)] on_toggle_star: Option<Callback<()>>,
    #[prop(optional)] star_tooltip: Option<&'static str>,
    #[prop(optional)] is_supernatural: Option<Signal<bool>>,
    #[prop(optional)] on_toggle_supernatural: Option<Callback<()>>,
    #[prop(optional)] suggested_specialties: Option<Signal<Vec<&'static str>>>,
    #[prop(optional)] on_open_compendium: Option<Callback<()>>,
    #[prop(into, optional)] compendium_page_ref: Option<MaybeSignal<String>>,
    #[prop(default = None)] on_flip: Option<Callback<()>>,
    #[prop(default = None)] flip_tooltip: Option<Signal<String>>,
) -> impl IntoView {
    let lang_ctx = use_context::<crate::i18n::LanguageContext>();
    let lang = Signal::derive(move || lang_ctx.map(|c| c.lang.get()).unwrap_or_default());

    // Os sinais recebidos costumam ser derivados da ficha inteira e recalculam a cada leitura.
    // Memorizados aqui, cada valor é calculado uma vez por edição, e uma mudança em outro
    // campo não acorda as bolinhas, dicas e selos deste.
    let (label, level, modifier) = (memo(label), memo(level), memo(modifier));
    let origins = origins.map(memo);
    let is_starred = is_starred.map(memo);
    let is_supernatural = is_supernatural.map(memo);

    let field = TraitField {
        label,
        level,
        modifier,
        origins,
        suggested_specialties,
        page_ref_str: Signal::derive(move || {
            compendium_page_ref.as_ref().map(|p| p.get()).unwrap_or_else(|| "M20, p. 273".to_string())
        }),
        modifier_ref: create_node_ref::<html::Input>(),
        last_modifier_value: create_rw_signal(String::new()),
        popover: create_rw_signal(None),
        on_level_change: Callback::new(on_level_change),
        on_modifier_change: Callback::new(on_modifier_change),
        on_origin_change: on_dot_origin_change,
        on_open_compendium,
    };

    let show_menu = create_rw_signal(false);
    let show_scale = create_rw_signal(false);
    let has_supernatural_toggle = on_toggle_supernatural.is_some();
    let has_supernatural_support = has_supernatural_toggle || is_supernatural.is_some();

    // Botão direito no nome abre o menu da escala sobrenatural
    let on_label_right_click = move |ev: ev::MouseEvent| {
        if has_supernatural_toggle {
            ev.prevent_default();
            ev.stop_propagation();
            show_menu.update(|v| *v = !*v);
        }
    };

    let name = if is_editable {
        view! { <EditableName label=label on_change=on_label_change /> }.into_view()
    } else {
        view! { <StaticName field=field.clone() max_chars=max_chars is_starred=is_starred /> }.into_view()
    };

    view! {
        <div class="attribute-row">
            <div
                class="tooltip-container label-cell-container"
                on:contextmenu=on_label_right_click
                title=move || if has_supernatural_toggle {
                    "Botão direito no nome: Alternar Característica Sobrenatural (6º Ponto)"
                } else {
                    ""
                }
            >
                {on_toggle_star.map(|on_star| view! { <AffinityStar is_starred=is_starred on_star=on_star tooltip=star_tooltip /> })}
                {on_flip.map(|on_flip| view! { <FlipButton on_flip=on_flip tooltip=flip_tooltip /> })}
                {name}
                <NameTooltip field=field.clone() />
                {on_toggle_supernatural.map(|on_toggle| view! {
                    <SupernaturalMenu
                        field=field.clone()
                        is_supernatural=is_supernatural
                        on_toggle=on_toggle
                        show_menu=show_menu
                        show_scale=show_scale
                        lang=lang
                    />
                })}
            </div>

            <ModifierCell field=field.clone() />
            <ValueDots field=field.clone() min_level=min_level max_level=max_level />
            {has_supernatural_support.then(|| view! { <SupernaturalSlot field=field.clone() show_scale=show_scale lang=lang /> })}

            <div class="action-area">
                {on_remove.map(|cb| view! {
                    <button class="remove-btn" on:click=move |_| cb.call(())>"×"</button>
                })}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_field_instantiation_and_reactive_updates() {
        let runtime = create_runtime();
        let (level, set_level) = create_signal(3);
        let (modifier, set_modifier) = create_signal("Test".to_string());
        let (is_starred, set_starred) = create_signal(false);
        let (_star_clicked, set_star_clicked) = create_signal(false);

        let on_toggle_star = Callback::new(move |_| {
            set_star_clicked.set(true);
            set_starred.update(|s| *s = !*s);
        });

        let _view = view! {
            <ValueField 
                label=Signal::derive(|| "Correspondência".to_string())
                level=level.into()
                modifier=modifier.into()
                on_level_change=move |v| set_level.set(v)
                on_modifier_change=move |m| set_modifier.set(m)
                is_starred=is_starred.into()
                on_toggle_star=on_toggle_star
                star_tooltip="Esfera de Afinidade"
            />
        };

        // Simulate signal mutation after initial render to ensure no panic
        set_starred.set(true);
        set_level.set(4);
        set_modifier.set("Especialização".to_string());

        assert_eq!(level.get(), 4);
        assert_eq!(modifier.get(), "Especialização");
        assert_eq!(is_starred.get(), true);

        // Test supernatural trait support
        let (is_supernatural, set_supernatural) = create_signal(false);
        let (_supernatural_toggled, set_supernatural_toggled) = create_signal(false);
        let on_toggle_supernatural = Callback::new(move |_| {
            set_supernatural_toggled.set(true);
            set_supernatural.update(|s| *s = !*s);
        });

        let _view_supernatural = view! {
            <ValueField 
                label=Signal::derive(|| "Força".to_string())
                level=level.into()
                modifier=modifier.into()
                on_level_change=move |v| set_level.set(v)
                on_modifier_change=move |m| set_modifier.set(m)
                is_supernatural=is_supernatural.into()
                on_toggle_supernatural=on_toggle_supernatural
            />
        };

        // Test that setting level to 5 activates the 6th slot automatically
        set_supernatural.set(false);
        set_level.set(5);
        assert_eq!(level.get(), 5);
        assert_eq!(is_supernatural.get(), false);

        set_supernatural.set(true);
        set_level.set(6);
        assert_eq!(level.get(), 6);
        assert_eq!(is_supernatural.get(), true);

        // Test supernatural levels 8 (+3) and 10 (+5)
        set_level.set(8);
        assert_eq!(level.get(), 8);

        set_level.set(10);
        assert_eq!(level.get(), 10);

        runtime.dispose();
    }

    #[test]
    fn test_value_field_with_specialties_and_compendium_page_ref() {
        let runtime = create_runtime();
        let (level, set_level) = create_signal(4);
        let (modifier, set_modifier) = create_signal("Musculoso".to_string());
        let (compendium_opened, set_compendium_opened) = create_signal(false);
        let on_open_compendium = Callback::new(move |_| {
            set_compendium_opened.set(true);
        });

        let specs = Signal::derive(|| vec!["Musculoso", "Atlético", "Resistente"]);

        let _view = view! {
            <ValueField 
                label=Signal::derive(|| "Força".to_string())
                level=level.into()
                modifier=modifier.into()
                on_level_change=move |v| set_level.set(v)
                on_modifier_change=move |m| set_modifier.set(m)
                suggested_specialties=specs
                compendium_page_ref="M20, p. 273"
                on_open_compendium=on_open_compendium.clone()
                is_supernatural=Signal::derive(|| true)
            />
        };

        assert_eq!(level.get(), 4);
        assert_eq!(modifier.get(), "Musculoso");
        assert_eq!(compendium_opened.get(), false);

        on_open_compendium.call(());
        assert_eq!(compendium_opened.get(), true);

        runtime.dispose();
    }
}

