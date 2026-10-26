use mta_sheet::logging::{
    server::{
        enforce_category_limits_with_max, get_active_log_file_with_limit, parse_log_file_key, CategoryFile,
        MAX_FILES_PER_CATEGORY, MAX_LOG_FILE_BYTES,
    },
    LogCategory,
};
use mta_sheet::server::is_ignorable_path;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_log_constants_match_spec() {
    assert_eq!(MAX_LOG_FILE_BYTES, 20 * 1024 * 1024, "Arquivo de log deve ter teto de 20 MB");
    assert_eq!(MAX_FILES_PER_CATEGORY, 5, "Cada categoria deve manter no máximo 5 arquivos");
    let total_category_cap = MAX_LOG_FILE_BYTES * (MAX_FILES_PER_CATEGORY as u64);
    assert_eq!(total_category_cap, 100 * 1024 * 1024, "Total máximo por categoria deve ser exatamente 100 MB");
}

#[test]
fn test_parse_log_file_key() {
    assert_eq!(
        parse_log_file_key("access", "access_2026-09-20.log"),
        ("2026-09-20".to_string(), 0)
    );
    assert_eq!(
        parse_log_file_key("access", "access_2026-09-20.1.log"),
        ("2026-09-20".to_string(), 1)
    );
    assert_eq!(
        parse_log_file_key("requests", "requests_2026-09-20.4.log"),
        ("2026-09-20".to_string(), 4)
    );
}

#[test]
fn test_asset_and_telemetry_filtering() {
    // Assets estáticos devem ser ignorados no log de acesso HTTP
    assert!(is_ignorable_path("/styles/00-tokens.css"));
    assert!(is_ignorable_path("/pkg/mta_sheet.wasm"));
    assert!(is_ignorable_path("/pkg/mta_sheet.js"));
    assert!(is_ignorable_path("/fonts/cinzel.woff2"));
    assert!(is_ignorable_path("/uploads/character.png"));
    assert!(is_ignorable_path("/banner.jpg"));
    assert!(is_ignorable_path("/favicon.ico"));
    assert!(is_ignorable_path("/favicon.svg"));
    assert!(is_ignorable_path("/api/record_client_log"));

    // Páginas reais, SSR e APIs devem ser registradas
    assert!(!is_ignorable_path("/"));
    assert!(!is_ignorable_path("/sheet/123"));
    assert!(!is_ignorable_path("/rooms"));
    assert!(!is_ignorable_path("/about"));
    assert!(!is_ignorable_path("/login"));
    assert!(!is_ignorable_path("/api/login"));
    assert!(!is_ignorable_path("/api/get_sheets"));
}

#[test]
fn test_log_slicing_when_size_limit_reached() {
    let cat = LogCategory::Access;
    let date_str = "2099-01-01";
    let limit_bytes = 200; // Limite baixo para teste unitário ágil

    // 1. Limpa arquivos de teste anteriores
    let base_file = format!("logs/access/access_{}.log", date_str);
    let slice_1 = format!("logs/access/access_{}.1.log", date_str);
    let slice_2 = format!("logs/access/access_{}.2.log", date_str);
    let _ = fs::remove_file(&base_file);
    let _ = fs::remove_file(&slice_1);
    let _ = fs::remove_file(&slice_2);

    // 2. Sem arquivos prévios -> deve apontar para o arquivo base
    let active = get_active_log_file_with_limit(&cat, date_str, limit_bytes);
    assert_eq!(active, base_file);

    // 3. Cria arquivo base com tamanho abaixo do limite -> permanece nele
    {
        let mut f = File::create(&base_file).unwrap();
        f.write_all(b"pequeno log de teste\n").unwrap();
    }
    let active = get_active_log_file_with_limit(&cat, date_str, limit_bytes);
    assert_eq!(active, base_file);

    // 4. Preenche arquivo base além do limite -> deve avançar para slice .1
    {
        let mut f = File::create(&base_file).unwrap();
        let big_chunk = vec![b'A'; 250];
        f.write_all(&big_chunk).unwrap();
    }
    let active = get_active_log_file_with_limit(&cat, date_str, limit_bytes);
    assert_eq!(active, slice_1);

    // 5. Preenche slice .1 além do limite -> deve avançar para slice .2
    {
        let mut f = File::create(&slice_1).unwrap();
        let big_chunk = vec![b'B'; 250];
        f.write_all(&big_chunk).unwrap();
    }
    let active = get_active_log_file_with_limit(&cat, date_str, limit_bytes);
    assert_eq!(active, slice_2);

    // 6. Limpeza
    let _ = fs::remove_file(&base_file);
    let _ = fs::remove_file(&slice_1);
    let _ = fs::remove_file(&slice_2);
}

#[test]
fn test_open_log_file_rotates_by_size_and_day_without_rescanning_each_line() {
    let cat = LogCategory::Errors;
    let (day1, day2) = ("2097-03-01", "2097-03-02");
    let file = |date: &str, slice: u32| {
        if slice == 0 {
            format!("logs/errors/errors_{}.log", date)
        } else {
            format!("logs/errors/errors_{}.{}.log", date, slice)
        }
    };
    for path in [file(day1, 0), file(day1, 1), file(day1, 2), file(day2, 0)] {
        let _ = fs::remove_file(path);
    }

    let mut writer = CategoryFile::new();
    let line = "[2097-03-01 00:00:00.000] [ERROR] linha de teste com uns 60 bytes\n";
    // Limite de 100 bytes: duas linhas por arquivo (a rotação acontece quando o arquivo já passou do limite)
    for _ in 0..4 {
        writer.append(&cat, day1, line, 100);
    }
    writer.flush();
    let base = fs::read_to_string(file(day1, 0)).expect("arquivo base");
    let slice = fs::read_to_string(file(day1, 1)).expect("fatia .1");
    assert_eq!(base.lines().count(), 2);
    assert_eq!(slice.lines().count(), 2);

    // Virada do dia: arquivo novo, mesmo com espaço sobrando
    writer.append(&cat, day2, line, 100);
    writer.flush();
    assert_eq!(fs::read_to_string(file(day2, 0)).expect("arquivo do dia 2").lines().count(), 1);

    for path in [file(day1, 0), file(day1, 1), file(day1, 2), file(day2, 0)] {
        let _ = fs::remove_file(path);
    }
}

#[test]
fn test_client_logs_cannot_forge_lines_or_categories() {
    use mta_sheet::logging::{sanitize_client_log, CLIENT_LOG_MAX_DETAILS, CLIENT_LOG_MAX_MESSAGE};

    // Quebra de linha viraria um registro falso no arquivo
    let forged = sanitize_client_log(
        "user_actions",
        "info",
        "ok\n[2026-01-01 00:00:00.000] [ERROR] admin apagou tudo",
        Some("a\r\nb"),
    );
    assert!(!forged.message.contains('\n') && !forged.message.contains('\r'));
    assert_eq!(forged.details.as_deref(), Some("a  b"));

    // O navegador não escreve no registro de acessos nem de requisições
    for category in ["access", "requests", "qualquer"] {
        assert_eq!(sanitize_client_log(category, "INFO", "x", None).category, LogCategory::UserActions);
    }
    assert_eq!(sanitize_client_log("errors", "ERROR", "x", None).category, LogCategory::Errors);

    // Nível fora da lista vira INFO; tamanho limitado
    assert_eq!(sanitize_client_log("errors", "CRITICAL", "x", None).level, "INFO");
    let huge = "x".repeat(10_000);
    let line = sanitize_client_log("errors", "WARN", &huge, Some(&huge));
    assert_eq!(line.message.chars().count(), CLIENT_LOG_MAX_MESSAGE);
    assert_eq!(line.details.as_deref().map(|d| d.chars().count()), Some(CLIENT_LOG_MAX_DETAILS));
}

#[test]
fn test_fifo_enforce_category_limits() {
    let cat = LogCategory::Database;
    let dir = format!("logs/{}", cat.as_str());
    let _ = fs::create_dir_all(&dir);

    // Cria 7 arquivos de teste simulados
    let mut files = Vec::new();
    for i in 1..=7 {
        let name = format!("{}/database_2098-01-0{}.log", dir, i);
        let mut f = File::create(&name).unwrap();
        writeln!(f, "Log line {}", i).unwrap();
        files.push(name);
        // Pequena pausa para garantir mtimes ordenados
        std::thread::sleep(std::time::Duration::from_millis(15));
    }

    // Aplica limite FIFO de no máximo 5 arquivos
    enforce_category_limits_with_max(&cat, 5);

    // Os 2 mais antigos (2098-01-01 e 2098-01-02) devem ter sido excluídos
    assert!(!std::path::Path::new(&files[0]).exists(), "Arquivo mais antigo 1 deve ter sido excluído");
    assert!(!std::path::Path::new(&files[1]).exists(), "Arquivo mais antigo 2 deve ter sido excluído");

    // Os 5 mais novos devem ter sido preservados
    for file in &files[2..] {
        assert!(std::path::Path::new(file).exists(), "Arquivo recente {} deve ser preservado", file);
        let _ = fs::remove_file(file);
    }
}
