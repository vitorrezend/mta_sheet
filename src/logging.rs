use serde::{Deserialize, Serialize};
use leptos::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LogCategory {
    Requests,
    Database,
    UserActions,
    Errors,
}

impl LogCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogCategory::Requests => "requests",
            LogCategory::Database => "database",
            LogCategory::UserActions => "user_actions",
            LogCategory::Errors => "errors",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            LogCategory::Requests => "Requisições (Requests)",
            LogCategory::Database => "Banco de Dados (Database)",
            LogCategory::UserActions => "Interações do Usuário (User Actions)",
            LogCategory::Errors => "Erros (Errors)",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogEntry {
    pub timestamp: String,
    pub category: String,
    pub level: String,
    pub message: String,
    pub details: Option<String>,
}

#[cfg(feature = "ssr")]
pub mod server {
    use super::*;
    use std::fs::{self, OpenOptions};
    use std::io::Write;
    use chrono::Local;

    pub fn get_log_dir(category: &LogCategory) -> String {
        format!("logs/{}", category.as_str())
    }

    pub fn ensure_log_dirs() {
        let categories = [
            LogCategory::Requests,
            LogCategory::Database,
            LogCategory::UserActions,
            LogCategory::Errors,
        ];
        for cat in &categories {
            let dir = get_log_dir(cat);
            let _ = fs::create_dir_all(&dir);
        }
    }

    pub fn write_log(category: LogCategory, level: &str, message: &str, details: Option<&str>) {
        ensure_log_dirs();
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let date_str = now.format("%Y-%m-%d").to_string();
        let file_path = format!("logs/{}/{}_{}.log", category.as_str(), category.as_str(), date_str);

        let details_str = details.map(|d| format!(" | {}", d)).unwrap_or_default();
        let line = format!("[{}] [{}] {}{}\n", timestamp, level.to_uppercase(), message, details_str);

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&file_path) {
            let _ = file.write_all(line.as_bytes());
        }

        // Also log to standard terminal
        match level.to_uppercase().as_str() {
            "ERROR" => log::error!("[{}] {}{}", category.as_str(), message, details_str),
            "WARN" => log::warn!("[{}] {}{}", category.as_str(), message, details_str),
            _ => log::info!("[{}] {}{}", category.as_str(), message, details_str),
        }
    }

    pub fn read_recent_logs(
        category_filter: Option<String>,
        search_filter: Option<String>,
        limit: usize,
    ) -> Vec<LogEntry> {
        ensure_log_dirs();
        let mut entries = Vec::new();
        let categories = match category_filter.as_deref() {
            Some("requests") => vec![LogCategory::Requests],
            Some("database") => vec![LogCategory::Database],
            Some("user_actions") => vec![LogCategory::UserActions],
            Some("errors") => vec![LogCategory::Errors],
            _ => vec![
                LogCategory::Requests,
                LogCategory::Database,
                LogCategory::UserActions,
                LogCategory::Errors,
            ],
        };

        for cat in categories {
            let dir = get_log_dir(&cat);
            if let Ok(dir_entries) = fs::read_dir(&dir) {
                let mut files: Vec<_> = dir_entries.filter_map(|e| e.ok()).collect();
                files.sort_by_key(|f| f.file_name());
                files.reverse(); // newest files first

                for file in files.into_iter().take(5) {
                    if let Ok(content) = fs::read_to_string(file.path()) {
                        for line in content.lines().rev() {
                            if line.trim().is_empty() {
                                continue;
                            }
                            if let Some(entry) = parse_log_line(cat.as_str(), line) {
                                if let Some(ref search) = search_filter {
                                    let s = search.to_lowercase();
                                    if !entry.message.to_lowercase().contains(&s)
                                        && !entry.details.as_deref().unwrap_or_default().to_lowercase().contains(&s)
                                    {
                                        continue;
                                    }
                                }
                                entries.push(entry);
                            }
                        }
                    }
                }
            }
        }

        // Sort by timestamp descending
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        entries.truncate(limit);
        entries
    }

    fn parse_log_line(category: &str, line: &str) -> Option<LogEntry> {
        // Line format: [2026-08-21 00:00:00.000] [LEVEL] Message | Details
        if !line.starts_with('[') {
            return None;
        }

        let parts: Vec<&str> = line.splitn(3, "] [").collect();
        if parts.len() < 2 {
            return None;
        }

        let timestamp = parts[0].trim_start_matches('[').to_string();
        let rest = parts[1];
        let level_msg: Vec<&str> = rest.splitn(2, "] ").collect();
        if level_msg.len() < 2 {
            return None;
        }

        let level = level_msg[0].to_string();
        let full_msg = level_msg[1];

        let (msg, details) = if let Some(idx) = full_msg.find(" | ") {
            (
                full_msg[..idx].to_string(),
                Some(full_msg[idx + 3..].to_string()),
            )
        } else {
            (full_msg.to_string(), None)
        };

        Some(LogEntry {
            timestamp,
            category: category.to_string(),
            level,
            message: msg,
            details,
        })
    }
}

// ==========================================
// Server Functions
// ==========================================

#[server(endpoint = "get_system_logs")]
pub async fn get_system_logs(
    category: Option<String>,
    search: Option<String>,
    limit: Option<usize>,
) -> Result<Vec<LogEntry>, ServerFnError> {
    let lim = limit.unwrap_or(200).min(500);
    Ok(server::read_recent_logs(category, search, lim))
}

#[server(endpoint = "record_client_log")]
pub async fn record_client_log(
    category: String,
    level: String,
    message: String,
    details: Option<String>,
) -> Result<(), ServerFnError> {
    let cat = match category.as_str() {
        "requests" => LogCategory::Requests,
        "database" => LogCategory::Database,
        "errors" => LogCategory::Errors,
        _ => LogCategory::UserActions,
    };
    server::write_log(cat, &level, &message, details.as_deref());
    Ok(())
}

/// Client & Server Logging Helper
pub fn log_client(category: &str, level: &str, message: &str, details: Option<&str>) {
    // Print to browser console with styling
    #[cfg(target_arch = "wasm32")]
    {
        let prefix = format!("[{}] [{}]", category.to_uppercase(), level.to_uppercase());
        let full_msg = match details {
            Some(d) => format!("{} {} | {}", prefix, message, d),
            None => format!("{} {}", prefix, message),
        };
        match level.to_uppercase().as_str() {
            "ERROR" => web_sys::console::error_1(&full_msg.into()),
            "WARN" => web_sys::console::warn_1(&full_msg.into()),
            _ => web_sys::console::log_1(&full_msg.into()),
        }
    }

    // Dispatch asynchronous record to server log file
    let c = category.to_string();
    let l = level.to_string();
    let m = message.to_string();
    let d = details.map(|s| s.to_string());
    spawn_local(async move {
        let _ = record_client_log(c, l, m, d).await;
    });
}
