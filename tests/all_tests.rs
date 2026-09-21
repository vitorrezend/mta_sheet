//! MTA Sheet — Suíte de Testes Unificada
//!
//! Todas as suítes de testes de integração do projeto são agrupadas aqui como submódulos
//! isolados. Isso reduz as invocações do linker de dezenas de binários separados para
//! exatamente 1 único binário (`all_tests`), acelerando dramaticamente a compilação local
//! e viabilizando a execução otimizada via `cargo-nextest`.

#![allow(dead_code, unused_imports)]

#[path = "suites/abilities_and_merits_flaws_test.rs"]
mod abilities_and_merits_flaws_test;

#[path = "suites/about_page_test.rs"]
mod about_page_test;

#[path = "suites/access_detection_test.rs"]
mod access_detection_test;

#[path = "suites/admin_auth_test.rs"]
mod admin_auth_test;

#[path = "suites/anti_patterns_test.rs"]
mod anti_patterns_test;

#[path = "suites/attributes_compendium_test.rs"]
mod attributes_compendium_test;

#[path = "suites/backgrounds_compendium_test.rs"]
mod backgrounds_compendium_test;

#[path = "suites/compact_json_and_quiz_test.rs"]
mod compact_json_and_quiz_test;

#[path = "suites/compendium_api_test.rs"]
mod compendium_api_test;

#[path = "suites/compendium_links_test.rs"]
mod compendium_links_test;

#[path = "suites/compendium_category_selector_test.rs"]
mod compendium_category_selector_test;

#[path = "suites/creation_points_test.rs"]
mod creation_points_test;

#[path = "suites/feature_flags_test.rs"]
mod feature_flags_test;

#[path = "suites/folder_acl_test.rs"]
mod folder_acl_test;

#[path = "suites/generate_compendium_json_test.rs"]
mod generate_compendium_json_test;

#[path = "suites/i18n_test.rs"]
mod i18n_test;

#[path = "suites/initiative_tracker_test.rs"]
mod initiative_tracker_test;

#[path = "suites/log_rotation_and_limits_test.rs"]
mod log_rotation_and_limits_test;

#[path = "suites/mobile_responsive_test.rs"]
mod mobile_responsive_test;

#[path = "suites/password_security_test.rs"]
mod password_security_test;

#[path = "suites/patch_notes_test.rs"]
mod patch_notes_test;

#[path = "suites/profile_and_feed_test.rs"]
mod profile_and_feed_test;

#[path = "suites/quiz_data_test.rs"]
mod quiz_data_test;

#[path = "suites/room_security_and_health_penalty_test.rs"]
mod room_security_and_health_penalty_test;

#[path = "suites/security_limits_test.rs"]
mod security_limits_test;

#[path = "suites/seo_and_home_showcase_test.rs"]
mod seo_and_home_showcase_test;

#[path = "suites/session_fifo_test.rs"]
mod session_fifo_test;

#[path = "suites/sheet_folders_test.rs"]
mod sheet_folders_test;

#[path = "suites/sheet_likes_test.rs"]
mod sheet_likes_test;

#[path = "suites/sheet_share_test.rs"]
mod sheet_share_test;

#[path = "suites/spheres_compendium_test.rs"]
mod spheres_compendium_test;

#[path = "suites/system_stats_test.rs"]
mod system_stats_test;

#[path = "suites/vitality_track_test.rs"]
mod vitality_track_test;

#[path = "suites/wasm_serving_test.rs"]
mod wasm_serving_test;

#[path = "suites/weapons_compendium_test.rs"]
mod weapons_compendium_test;

#[path = "suites/wonders_lifecycle_test.rs"]
mod wonders_lifecycle_test;

#[path = "suites/wysiwyg_editor_test.rs"]
mod wysiwyg_editor_test;
