use leptos::*;
use crate::compendium::weapons::WeaponDefinition;
use crate::components::compendium::{CompendiumModal, CompendiumSection};
use crate::components::Callback;

#[component]
pub fn WeaponCompendiumModal(
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
    #[prop(into)] initial_query: Signal<String>,
    #[prop(optional)] target_slot: Option<Signal<Option<usize>>>,
    #[prop(optional)] on_select_weapon: Option<Callback<(Option<usize>, &'static WeaponDefinition)>>,
) -> impl IntoView {
    let initial_sec = Signal::derive(|| CompendiumSection::Weapons);
    view! {
        <CompendiumModal
            show_modal=show_modal
            set_show_modal=set_show_modal
            initial_query=initial_query
            initial_section=Some(initial_sec)
            target_slot=target_slot
            on_select_weapon=on_select_weapon
        />
    }
}
