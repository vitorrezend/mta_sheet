use serde::{Deserialize, Serialize};
use leptos::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LogCategory {
    Requests,
    Database,
    UserActions,
    Errors,
    Access,
}

impl LogCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogCategory::Requests => "requests",
            LogCategory::Database => "database",
            LogCategory::UserActions => "user_actions",
            LogCategory::Errors => "errors",
            LogCategory::Access => "access",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            LogCategory::Requests => "Requisições (Requests)",
            LogCategory::Database => "Banco de Dados (Database)",
            LogCategory::UserActions => "Interações do Usuário (User Actions)",
            LogCategory::Errors => "Erros (Errors)",
            LogCategory::Access => "🛡️ Acessos (Humano vs Bot)",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClientType {
    Human,
    KnownBot,
    AutomatedScript,
    Suspicious,
}

impl ClientType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ClientType::Human => "human",
            ClientType::KnownBot => "crawler",
            ClientType::AutomatedScript => "bot",
            ClientType::Suspicious => "suspicious",
        }
    }

    pub fn display_label(&self) -> &'static str {
        match self {
            ClientType::Human => "🟢 HUMANO",
            ClientType::KnownBot => "🤖 CRAWLER",
            ClientType::AutomatedScript => "🔴 SCRIPT / BOT",
            ClientType::Suspicious => "⚠️ SUSPEITO",
        }
    }

    pub fn badge_class(&self) -> &'static str {
        match self {
            ClientType::Human => "badge-human",
            ClientType::KnownBot => "badge-crawler",
            ClientType::AutomatedScript => "badge-bot",
            ClientType::Suspicious => "badge-suspicious",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "human" => ClientType::Human,
            "crawler" => ClientType::KnownBot,
            "bot" => ClientType::AutomatedScript,
            _ => ClientType::Suspicious,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AccessClassification {
    pub client_type: ClientType,
    pub browser_os: String,
    pub confidence: u8,
    pub reason: String,
}

/// Classificador de tráfego (Humano vs Crawler vs Script Automatizado)
pub fn classify_traffic(
    user_agent_opt: Option<&str>,
    sec_fetch_mode: Option<&str>,
    sec_ch_ua: Option<&str>,
    accept_lang_opt: Option<&str>,
) -> AccessClassification {
    let ua = user_agent_opt.unwrap_or("").trim();

    // 1. Sem User-Agent -> Script de automação
    if ua.is_empty() {
        return AccessClassification {
            client_type: ClientType::AutomatedScript,
            browser_os: "Desconhecido (Sem User-Agent)".to_string(),
            confidence: 99,
            reason: "Requisição sem cabeçalho User-Agent".to_string(),
        };
    }

    let ua_lower = ua.to_lowercase();

    // 2. Assinaturas de ferramentas de automação e scripts
    let bot_tools = [
        ("curl", "curl"),
        ("python-requests", "Python Requests"),
        ("python-urllib", "Python urllib"),
        ("aiohttp", "Python aiohttp"),
        ("httpx", "Python HTTPX"),
        ("postmanruntime", "Postman"),
        ("insomnia", "Insomnia"),
        ("go-http-client", "Go HTTP Client"),
        ("apache-httpclient", "Apache HttpClient"),
        ("okhttp", "OkHttp"),
        ("node-fetch", "Node Fetch"),
        ("axios", "Axios (Node.js)"),
        ("scrapy", "Scrapy Web Scraper"),
        ("headlesschrome", "Headless Chrome"),
        ("phantomjs", "PhantomJS"),
        ("selenium", "Selenium Web Driver"),
        ("playwright", "Playwright Automation"),
        ("puppeteer", "Puppeteer Automation"),
        ("wget", "GNU Wget"),
        ("httpie", "HTTPie"),
        ("sqlmap", "SQLMap Security Tool"),
        ("nmap", "Nmap Scanner"),
        ("nikto", "Nikto Scanner"),
        ("zgrab", "ZGrab Scanner"),
        ("masscan", "Masscan Scanner"),
    ];

    for (pattern, name) in bot_tools {
        if ua_lower.contains(pattern) {
            return AccessClassification {
                client_type: ClientType::AutomatedScript,
                browser_os: name.to_string(),
                confidence: 99,
                reason: format!("Assinatura de ferramenta automatizada ({}) detectada", name),
            };
        }
    }

    // 3. Assinaturas de robôs de busca legítimos (Crawlers)
    let crawlers = [
        ("googlebot", "Googlebot (Google)"),
        ("bingbot", "Bingbot (Microsoft)"),
        ("duckduckbot", "DuckDuckBot"),
        ("yandexbot", "YandexBot"),
        ("baiduspider", "Baidu Spider"),
        ("facebookexternalhit", "Facebook Crawler"),
        ("twitterbot", "TwitterBot"),
        ("discordbot", "DiscordBot"),
        ("telegrambot", "TelegramBot"),
        ("slackbot", "SlackBot"),
        ("whatsapp", "WhatsApp Crawler"),
        ("applebot", "Applebot"),
        ("semrushbot", "Semrush Bot"),
        ("ahrefsbot", "Ahrefs Bot"),
        ("dotbot", "DotBot"),
        ("petalbot", "PetalBot (Huawei)"),
    ];

    for (pattern, name) in crawlers {
        if ua_lower.contains(pattern) {
            return AccessClassification {
                client_type: ClientType::KnownBot,
                browser_os: name.to_string(),
                confidence: 98,
                reason: format!("Identificado como robô indexador legítimo ({})", name),
            };
        }
    }

    // 4. Detecção de Sistema Operacional
    let os = if ua_lower.contains("windows") {
        "Windows"
    } else if ua_lower.contains("android") {
        "Android"
    } else if ua_lower.contains("iphone") || ua_lower.contains("ipad") {
        "iOS"
    } else if ua_lower.contains("macintosh") || ua_lower.contains("mac os") {
        "macOS"
    } else if ua_lower.contains("linux") {
        "Linux"
    } else if ua_lower.contains("cros") {
        "ChromeOS"
    } else {
        "Outro SO"
    };

    // 5. Detecção de Navegador
    let browser = if ua_lower.contains("edg/") || ua_lower.contains("edge/") {
        "Edge"
    } else if ua_lower.contains("opr/") || ua_lower.contains("opera") {
        "Opera"
    } else if ua_lower.contains("chrome") && !ua_lower.contains("chromium") {
        "Chrome"
    } else if ua_lower.contains("firefox") {
        "Firefox"
    } else if ua_lower.contains("safari") && !ua_lower.contains("chrome") {
        "Safari"
    } else {
        "Navegador Genérico"
    };

    let browser_os = format!("{} ({})", browser, os);

    // 6. Heurística de Headers de Navegador Real
    let has_sec_fetch = sec_fetch_mode.is_some();
    let has_sec_ch_ua = sec_ch_ua.is_some();
    let has_accept_lang = accept_lang_opt.is_some();

    if has_sec_fetch || has_sec_ch_ua || has_accept_lang {
        let mut score = 85u8;
        if has_sec_fetch { score += 5; }
        if has_sec_ch_ua { score += 5; }
        if has_accept_lang { score += 4; }

        AccessClassification {
            client_type: ClientType::Human,
            browser_os,
            confidence: score.min(99),
            reason: "Cabeçalhos de navegador modernos consistentes com usuário humano".to_string(),
        }
    } else if ua_lower.contains("mozilla/5.0") {
        // User-Agent finge ser Mozilla, mas não possui nenhum header típico de navegador moderno
        AccessClassification {
            client_type: ClientType::Suspicious,
            browser_os,
            confidence: 75,
            reason: "User-Agent alega ser navegador, mas faltam cabeçalhos típicos (Sec-Fetch/Accept-Language)".to_string(),
        }
    } else {
        AccessClassification {
            client_type: ClientType::Suspicious,
            browser_os,
            confidence: 60,
            reason: "Padrão de cabeçalhos atípico".to_string(),
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
            LogCategory::Access,
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

    pub fn write_access_log(
        method: &str,
        path: &str,
        status: u16,
        duration_ms: u128,
        ip: &str,
        classification: &AccessClassification,
    ) {
        let level = match classification.client_type {
            ClientType::Human => "INFO",
            ClientType::KnownBot => "INFO",
            ClientType::AutomatedScript => "WARN",
            ClientType::Suspicious => "WARN",
        };

        let message = format!("{} {} -> {} ({}ms)", method, path, status, duration_ms);
        let details = format!(
            "type:{} | ip:{} | browser:{} | score:{}% | reason:{}",
            classification.client_type.as_str(),
            ip,
            classification.browser_os,
            classification.confidence,
            classification.reason
        );

        write_log(LogCategory::Access, level, &message, Some(&details));
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
            Some("access") => vec![LogCategory::Access],
            _ => vec![
                LogCategory::Access,
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
    let is_admin = crate::auth::is_current_user_admin().await?;
    if !is_admin {
        return Err(ServerFnError::new(
            "Acesso negado. Apenas administradores do sistema podem visualizar os logs de auditoria."
        ));
    }
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
        "access" => LogCategory::Access,
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
