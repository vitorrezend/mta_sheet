//! Motor de regras puras do MTA Sheet (Mago: A Ascensão M20 & World of Darkness)
//!
//! Este módulo encapsula regras de negócio, tabelas matemáticas e cálculos de RPG
//! de forma 100% pura, agnóstica de banco de dados, servidor HTTP e UI reativa.

pub mod health;
pub mod quintessence;
pub mod initiative;

pub use health::*;
pub use quintessence::*;
pub use initiative::*;
