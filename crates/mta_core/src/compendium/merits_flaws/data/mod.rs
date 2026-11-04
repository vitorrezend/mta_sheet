//! Catálogo Unificado de Qualidades, Defeitos e Perturbações (M20).

pub mod merits;
pub mod flaws;
pub mod derangements;
pub mod bos;

pub use merits::ALL_MERITS;
pub use flaws::ALL_FLAWS;
pub use derangements::ALL_DERANGEMENTS;
pub use bos::{BOS_FLAWS, BOS_MERITS};
