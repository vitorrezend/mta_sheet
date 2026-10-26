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

    pub const MAX_LOG_FILE_BYTES: u64 = 20 * 1024 * 1024; // 20 MB por arquivo
    pub const MAX_FILES_PER_CATEGORY: usize = 5; // Máximo de 5 arquivos por categoria (total de 100 MB)

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

    /// Extrai (data, número_do_fatiamento) a partir do nome do arquivo para ordenação cronológica precisa
    pub fn parse_log_file_key(category: &str, file_name: &str) -> (String, u32) {
        let prefix = format!("{}_", category);
        if let Some(rest) = file_name.strip_prefix(&prefix).and_then(|s| s.strip_suffix(".log")) {
            if let Some((date_part, slice_part)) = rest.split_once('.') {
                if let Ok(slice) = slice_part.parse::<u32>() {
                    return (date_part.to_string(), slice);
                }
            }
            return (rest.to_string(), 0);
        }
        (file_name.to_string(), 0)
    }

    /// Retorna o caminho do arquivo de log ativo para escrita, fatiando se exceder `max_bytes`
    pub fn get_active_log_file_with_limit(category: &LogCategory, date_str: &str, max_bytes: u64) -> String {
        let dir = get_log_dir(category);
        let prefix = format!("{}_{}", category.as_str(), date_str);

        let mut highest_slice: Option<(u32, std::path::PathBuf)> = None;

        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if !name.ends_with(".log") {
                    continue;
                }
                if name == format!("{}.log", prefix) {
                    if highest_slice.is_none() {
                        highest_slice = Some((0, entry.path()));
                    }
                } else if let Some(slice_part) = name.strip_prefix(&format!("{}.", prefix)).and_then(|s| s.strip_suffix(".log")) {
                    if let Ok(idx) = slice_part.parse::<u32>() {
                        if highest_slice.as_ref().map_or(true, |(h_idx, _)| idx > *h_idx) {
                            highest_slice = Some((idx, entry.path()));
                        }
                    }
                }
            }
        }

        match highest_slice {
            None => format!("{}/{}.log", dir, prefix),
            Some((idx, path)) => {
                if let Ok(meta) = fs::metadata(&path) {
                    if meta.len() < max_bytes {
                        let filename = if idx == 0 {
                            format!("{}.log", prefix)
                        } else {
                            format!("{}.{}.log", prefix, idx)
                        };
                        return format!("{}/{}", dir, filename);
                    }
                }
                // Arquivo atual atingiu o limite de bytes: avança para o próximo fatiamento
                format!("{}/{}.{}.log", dir, prefix, idx + 1)
            }
        }
    }

    /// Obtém o arquivo de log ativo usando o limite padrão de 20 MB por arquivo
    pub fn get_active_log_file(category: &LogCategory, date_str: &str) -> String {
        get_active_log_file_with_limit(category, date_str, MAX_LOG_FILE_BYTES)
    }

    /// Garante que nenhuma categoria ultrapasse o número máximo de arquivos (FIFO: deleta os mais antigos)
    pub fn enforce_category_limits_with_max(category: &LogCategory, max_files: usize) {
        let dir = get_log_dir(category);
        if let Ok(entries) = fs::read_dir(&dir) {
            let mut log_files: Vec<(std::path::PathBuf, std::time::SystemTime, (String, u32))> = Vec::new();
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.ends_with(".log") {
                        let mtime = entry.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                        let key = parse_log_file_key(category.as_str(), &name);
                        log_files.push((path, mtime, key));
                    }
                }
            }

            if log_files.len() > max_files {
                // Ordena do mais antigo para o mais recente: primeiro por mtime, desempate por (data, slice)
                log_files.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.2.cmp(&b.2)));
                let to_remove = log_files.len() - max_files;
                for (path, _, _) in log_files.into_iter().take(to_remove) {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }

    /// Garante o limite padrão de no máximo 5 arquivos por categoria (FIFO: max 100 MB)
    pub fn enforce_category_limits(category: &LogCategory) {
        enforce_category_limits_with_max(category, MAX_FILES_PER_CATEGORY);
    }

    /// Aplica o limite FIFO em todas as categorias de logs do sistema
    pub fn enforce_all_category_limits() {
        let categories = [
            LogCategory::Requests,
            LogCategory::Database,
            LogCategory::UserActions,
            LogCategory::Errors,
            LogCategory::Access,
        ];
        for cat in &categories {
            enforce_category_limits(cat);
        }
    }

    pub fn cleanup_old_logs(max_days: i64) {
        let categories = [
            LogCategory::Requests,
            LogCategory::Database,
            LogCategory::UserActions,
            LogCategory::Errors,
            LogCategory::Access,
        ];
        let cutoff = Local::now() - chrono::Duration::days(max_days);
        let cutoff_str = cutoff.format("%Y-%m-%d").to_string();

        for cat in &categories {
            let dir = get_log_dir(cat);
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.ends_with(".log") {
                        let (date_part, _) = parse_log_file_key(cat.as_str(), &file_name);
                        if date_part.as_str() < cutoff_str.as_str() {
                            let _ = fs::remove_file(entry.path());
                        }
                    }
                }
            }
        }
    }

    /// Arquivo de log aberto de uma categoria, com o tamanho acompanhado em memória.
    ///
    /// A pasta só é listada (para achar o arquivo da vez) quando o dia muda ou o arquivo
    /// enche; no resto do tempo cada linha é só uma escrita no arquivo já aberto.
    pub struct CategoryFile {
        date: String,
        size: u64,
        file: Option<std::io::BufWriter<fs::File>>,
    }

    impl CategoryFile {
        pub fn new() -> Self {
            Self { date: String::new(), size: 0, file: None }
        }

        /// Escreve uma linha (sem flush), girando o arquivo se o dia mudou ou passou de `max_bytes`.
        pub fn append(&mut self, category: &LogCategory, date: &str, line: &str, max_bytes: u64) {
            if self.file.is_none() || self.date != date || self.size >= max_bytes {
                self.flush();
                let _ = fs::create_dir_all(get_log_dir(category));
                let path = get_active_log_file_with_limit(category, date, max_bytes);
                self.file = OpenOptions::new().create(true).append(true).open(&path).ok().map(std::io::BufWriter::new);
                self.size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                self.date = date.to_string();
                // Abrir um arquivo novo é o único momento em que a contagem de arquivos muda
                enforce_category_limits(category);
            }
            if let Some(file) = self.file.as_mut()
                && file.write_all(line.as_bytes()).is_ok()
            {
                self.size += line.len() as u64;
            }
        }

        pub fn flush(&mut self) {
            if let Some(file) = self.file.as_mut() {
                let _ = file.flush();
            }
        }
    }

    impl Default for CategoryFile {
        fn default() -> Self {
            Self::new()
        }
    }

    struct LogLine {
        category: LogCategory,
        date: String,
        line: String,
    }

    /// Canal para a thread que grava os logs. Criada no primeiro uso.
    static LOG_SENDER: std::sync::OnceLock<std::sync::Mutex<std::sync::mpsc::Sender<LogLine>>> = std::sync::OnceLock::new();

    fn log_sender() -> &'static std::sync::Mutex<std::sync::mpsc::Sender<LogLine>> {
        LOG_SENDER.get_or_init(|| {
            let (sender, receiver) = std::sync::mpsc::channel::<LogLine>();
            let spawned = std::thread::Builder::new().name("mta-log-writer".into()).spawn(move || {
                let mut files: std::collections::HashMap<&'static str, CategoryFile> = std::collections::HashMap::new();
                // Bloqueia até chegar uma linha, grava tudo que já estiver na fila e só então faz flush
                while let Ok(first) = receiver.recv() {
                    let mut batch = vec![first];
                    batch.extend(receiver.try_iter().take(1024));
                    for entry in batch {
                        files
                            .entry(entry.category.as_str())
                            .or_default()
                            .append(&entry.category, &entry.date, &entry.line, MAX_LOG_FILE_BYTES);
                    }
                    files.values_mut().for_each(CategoryFile::flush);
                }
            });
            if let Err(e) = spawned {
                log::error!("Não foi possível iniciar a thread de logs: {}", e);
            }
            std::sync::Mutex::new(sender)
        })
    }

    /// Registra uma linha de log. Não toca no disco: a thread de logs grava em segundo plano,
    /// então pode ser chamada de qualquer lugar sem bloquear as threads que atendem requisições.
    pub fn write_log(category: LogCategory, level: &str, message: &str, details: Option<&str>) {
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let date = now.format("%Y-%m-%d").to_string();

        let details_str = details.map(|d| format!(" | {}", d)).unwrap_or_default();
        let line = format!("[{}] [{}] {}{}\n", timestamp, level.to_uppercase(), message, details_str);

        if let Ok(sender) = log_sender().lock() {
            let _ = sender.send(LogLine { category: category.clone(), date, line });
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

    /// Lê de forma otimizada os últimos bytes do arquivo (estilo tail) para evitar carregar dezenas de MBs na RAM
    fn read_file_tail(path: &std::path::Path, max_bytes: u64) -> std::io::Result<String> {
        use std::io::{Read, Seek, SeekFrom};
        let mut file = fs::File::open(path)?;
        let file_len = file.metadata()?.len();
        if file_len == 0 {
            return Ok(String::new());
        }

        let bytes_to_read = file_len.min(max_bytes);
        if bytes_to_read < file_len {
            file.seek(SeekFrom::End(-(bytes_to_read as i64)))?;
        }

        let mut buffer = vec![0u8; bytes_to_read as usize];
        file.read_exact(&mut buffer)?;

        let text = String::from_utf8_lossy(&buffer);
        if bytes_to_read < file_len {
            // Se buscou no meio do arquivo, descarta a primeira linha incompleta
            if let Some(pos) = text.find('\n') {
                return Ok(text[pos + 1..].to_string());
            }
        }
        Ok(text.into_owned())
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
                // Ordena os arquivos mais recentes primeiro: mtime decrescente, desempate por chave decrescente
                files.sort_by(|a, b| {
                    let mtime_a = a.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    let mtime_b = b.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    let key_a = parse_log_file_key(cat.as_str(), &a.file_name().to_string_lossy());
                    let key_b = parse_log_file_key(cat.as_str(), &b.file_name().to_string_lossy());
                    mtime_b.cmp(&mtime_a).then_with(|| key_b.cmp(&key_a))
                });

                for file in files.into_iter().take(5) {
                    if let Ok(content) = read_file_tail(&file.path(), 512 * 1024) {
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
                                if entries.len() >= limit * 2 {
                                    break;
                                }
                            }
                        }
                    }
                    if entries.len() >= limit * 2 {
                        break;
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
    // Lê até 5 arquivos do disco: fora das threads que atendem requisições
    tokio::task::spawn_blocking(move || server::read_recent_logs(category, search, lim))
        .await
        .map_err(|e| ServerFnError::new(format!("Falha ao ler os logs: {}", e)))
}

/// Tamanho máximo da mensagem e dos detalhes de um log enviado pelo navegador
pub const CLIENT_LOG_MAX_MESSAGE: usize = 300;
pub const CLIENT_LOG_MAX_DETAILS: usize = 1000;
/// Logs enviados por usuário por minuto
pub const CLIENT_LOGS_PER_MINUTE: usize = 60;

/// Uma linha de log vinda do navegador, já limpa.
#[derive(Debug, PartialEq)]
pub struct ClientLogLine {
    pub category: LogCategory,
    pub level: &'static str,
    pub message: String,
    pub details: Option<String>,
}

/// Uma linha de texto só, sem caracteres de controle e com tamanho limitado.
/// Sem isso, um `\n` na mensagem criaria linhas falsas no arquivo de log.
fn single_line(text: &str, max_chars: usize) -> String {
    text.chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .take(max_chars)
        .collect::<String>()
        .trim()
        .to_string()
}

/// Limpa o log enviado pelo navegador. O navegador só escreve nas categorias de uso
/// (ações, erros, banco); acessos e requisições são registrados pelo servidor.
pub fn sanitize_client_log(category: &str, level: &str, message: &str, details: Option<&str>) -> ClientLogLine {
    let category = match category {
        "errors" => LogCategory::Errors,
        "database" => LogCategory::Database,
        _ => LogCategory::UserActions,
    };
    let level = match level.to_ascii_uppercase().as_str() {
        "ERROR" => "ERROR",
        "WARN" => "WARN",
        _ => "INFO",
    };
    ClientLogLine {
        category,
        level,
        message: single_line(message, CLIENT_LOG_MAX_MESSAGE),
        details: details.map(|d| single_line(d, CLIENT_LOG_MAX_DETAILS)).filter(|d| !d.is_empty()),
    }
}

#[server(endpoint = "record_client_log")]
pub async fn record_client_log(
    category: String,
    level: String,
    message: String,
    details: Option<String>,
) -> Result<(), ServerFnError> {
    // Só de quem está logado e com limite: sem isso, qualquer um enchia o disco de logs.
    // Visitantes são ignorados em silêncio (o navegador registra para todos)
    let Some(user_id) = crate::auth::get_auth_user_id().await? else { return Ok(()) };
    if !crate::server::middleware::security::check_action_rate_limit(
        &format!("client_log:{}", user_id),
        CLIENT_LOGS_PER_MINUTE,
        std::time::Duration::from_secs(60),
    ) {
        return Ok(());
    }
    let line = sanitize_client_log(&category, &level, &message, details.as_deref());
    let details = match line.details {
        Some(d) => format!("user:{} | {}", user_id, d),
        None => format!("user:{}", user_id),
    };
    server::write_log(line.category, line.level, &line.message, Some(&details));
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
