//! Erros de domínio do servidor, com mensagens consistentes para quem usa o app.
//!
//! Padrão para código novo: funções auxiliares retornam [`AppResult`] e as server
//! functions usam `?` normalmente (há conversão automática para `ServerFnError`).
//! Erros internos (banco, serialização) vão para o log com o detalhe técnico; o usuário
//! vê só uma mensagem genérica, sem SQL nem nomes de tabelas.

use std::fmt;

/// Prefixo que o cliente reconhece para oferecer "recarregar" ou "manter minhas alterações".
pub const VERSION_CONFLICT_PREFIX: &str = "CONFLITO_DE_VERSAO";

#[derive(Debug)]
pub enum AppError {
    /// Precisa estar logado
    NotLoggedIn,
    /// Logado, mas sem permissão (mensagem explica o motivo)
    Forbidden(String),
    NotFound(String),
    /// Alguém salvou antes (concorrência otimista)
    VersionConflict(String),
    /// Dados enviados inválidos
    Invalid(String),
    RateLimited(String),
    /// Erro inesperado: o detalhe vai para o log, nunca para o usuário
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::Forbidden(msg.into())
    }
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
    pub fn invalid(msg: impl Into<String>) -> Self {
        Self::Invalid(msg.into())
    }
    /// Registra o detalhe técnico no log e devolve um erro genérico ao usuário.
    pub fn internal(context: &str, detail: impl fmt::Display) -> Self {
        log::error!("{}: {}", context, detail);
        Self::Internal(context.to_string())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotLoggedIn => write!(f, "Você precisa estar logado para fazer isso"),
            Self::Forbidden(msg) | Self::NotFound(msg) | Self::Invalid(msg) | Self::RateLimited(msg) => write!(f, "{}", msg),
            Self::VersionConflict(msg) => write!(f, "{}: {}", VERSION_CONFLICT_PREFIX, msg),
            Self::Internal(_) => write!(f, "Erro interno no servidor. Tente novamente em instantes."),
        }
    }
}

/// Também dá a conversão para `ServerFnError`: o Leptos converte qualquer
/// `std::error::Error` usando a mensagem de `Display` acima.
impl std::error::Error for AppError {}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::internal("Erro de banco de dados", err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::ServerFnError;

    #[test]
    fn internal_errors_do_not_leak_details() {
        let err = AppError::internal("Falha ao salvar ficha", "UNIQUE constraint failed: character_sheets.id");
        let shown: ServerFnError = err.into();
        let shown = shown.to_string();
        assert!(!shown.contains("constraint"), "detalhe técnico não pode chegar ao usuário: {shown}");
    }

    #[test]
    fn version_conflict_keeps_client_prefix() {
        let shown = AppError::VersionConflict("outra pessoa salvou antes".into()).to_string();
        assert!(shown.starts_with(VERSION_CONFLICT_PREFIX));
    }
}
