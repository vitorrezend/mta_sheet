//! Domínio do MTA Sheet (Mago: A Ascensão M20): regras puras, modelos da ficha e compêndio.
//!
//! Não depende de Leptos, Axum nem banco de dados. Fica num crate próprio para que o
//! compilador garanta essa fronteira e para que mudanças na interface não recompilem
//! as regras e os dados do compêndio.

pub mod compendium;
pub mod costs;
pub mod i18n;
pub mod json_patch;
pub mod models;
pub mod rich_text;
pub mod rules;
pub mod sanitization;
