// Modelos da ficha e as regras sobre eles moram no crate de domínio (crates/mta_core)
pub use mta_core::{costs, json_patch, models, rich_text, sanitization};
pub mod server_fns;
pub mod server_fns_folders;
pub mod server_fns_share;
pub mod server_fns_save;

#[cfg(test)]
mod tests;

// Re-export all models, constants, and helpers
pub use models::*;
pub use server_fns::*;
#[allow(unused_imports)]
pub use server_fns_folders::*;
#[allow(unused_imports)]
pub use server_fns_share::*;
pub use server_fns_save::*;
