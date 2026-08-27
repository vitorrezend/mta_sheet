use mta_sheet::components::common::patch_notes_data::{PATCH_RELEASES, CURRENT_VERSION, get_latest_release};
use mta_sheet::components::common::patch_notes_modal::PatchNotesModal;
use mta_sheet::components::SafeCallback;
use leptos::*;

#[test]
fn test_current_version_matches_cargo_toml() {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Cargo.toml deve existir");
    let version_line = cargo_toml.lines().find(|l| l.trim().starts_with("version")).expect("Versão deve existir no Cargo.toml");
    let expected_version = version_line.split('=').nth(1).unwrap().trim().trim_matches('"');

    assert_eq!(
        CURRENT_VERSION, expected_version,
        "A constante CURRENT_VERSION ({}) deve bater com a versão no Cargo.toml ({})",
        CURRENT_VERSION, expected_version
    );
}

#[test]
fn test_patch_releases_data_integrity() {
    assert!(!PATCH_RELEASES.is_empty(), "PATCH_RELEASES não pode estar vazio");

    for release in PATCH_RELEASES {
        assert!(!release.version.is_empty(), "Versão não pode ser vazia");
        assert!(!release.date.is_empty(), "Data não pode ser vazia");
        assert!(!release.title.is_empty(), "Título não pode ser vazio");
        assert!(!release.highlight.is_empty(), "Destaque não pode ser vazio");
        assert!(!release.sections.is_empty(), "Release {} deve conter seções", release.version);

        for section in release.sections {
            assert!(!section.category.is_empty(), "Categoria não pode ser vazia");
            assert!(!section.icon.is_empty(), "Ícone não pode ser vazio");
            assert!(!section.items.is_empty(), "Seção {} deve conter itens", section.category);

            for item in section.items {
                assert!(!item.trim().is_empty(), "Item de patch note não pode ser vazio");
            }
        }
    }
}

#[test]
fn test_latest_release_matches_current_version() {
    let latest = get_latest_release().expect("Deve haver ao menos uma release");
    let stripped_v = latest.version.trim_start_matches('v');
    assert_eq!(
        stripped_v, CURRENT_VERSION,
        "A última release cadastrada ({}) deve corresponder à versão do pacote ({})",
        latest.version, CURRENT_VERSION
    );
}

#[test]
fn test_patch_notes_modal_ssr_rendering() {
    let html = leptos::ssr::render_to_string(|| {
        let (is_open, set_is_open) = create_signal(true);
        let on_close = SafeCallback::new(move |_| set_is_open.set(false));
        view! {
            <PatchNotesModal is_open=is_open on_close=on_close />
        }
    });

    assert!(!html.is_empty(), "Modal de Patch Notes deve renderizar HTML válido");
    assert!(html.contains("Notas de Atualização"), "HTML deve conter o título do modal");
    assert!(html.contains("v0.10.0"), "HTML deve conter a versão v0.10.0");
    assert!(html.contains("patch-modal-overlay"), "HTML deve conter a classe patch-modal-overlay");
}

#[test]
fn test_patch_notes_modal_scope_disposal_safety() {
    let runtime = create_runtime();
    let (is_open, set_is_open) = create_signal(true);
    let on_close = SafeCallback::new(move |_| set_is_open.set(false));

    let _view = view! {
        <PatchNotesModal is_open=is_open on_close=on_close />
    };

    // Descarta o runtime simulando a saida de tela/desmontagem
    runtime.dispose();

    // try_get_untracked nao pode panicar mesmo apos o runtime ser descartado
    assert_eq!(is_open.try_get_untracked(), None);
}

