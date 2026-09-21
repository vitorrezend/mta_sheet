use mta_sheet::logging::{classify_traffic, ClientType};

#[test]
fn test_classify_chrome_human() {
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";
    let res = classify_traffic(
        Some(ua),
        Some("navigate"),
        Some("\"Chromium\";v=\"124\", \"Google Chrome\";v=\"124\""),
        Some("pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7"),
    );

    assert_eq!(res.client_type, ClientType::Human);
    assert!(res.confidence >= 95);
    assert!(res.browser_os.contains("Chrome"));
    assert!(res.browser_os.contains("Windows"));
}

#[test]
fn test_classify_safari_iphone_human() {
    let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4.1 Mobile/15E148 Safari/604.1";
    let res = classify_traffic(
        Some(ua),
        Some("navigate"),
        None,
        Some("pt-BR,pt;q=0.9"),
    );

    assert_eq!(res.client_type, ClientType::Human);
    assert!(res.confidence >= 90);
    assert!(res.browser_os.contains("Safari"));
    assert!(res.browser_os.contains("iOS"));
}

#[test]
fn test_classify_curl_bot() {
    let ua = "curl/8.4.0";
    let res = classify_traffic(Some(ua), None, None, None);

    assert_eq!(res.client_type, ClientType::AutomatedScript);
    assert_eq!(res.confidence, 99);
    assert!(res.browser_os.contains("curl"));
}

#[test]
fn test_classify_python_requests_bot() {
    let ua = "python-requests/2.31.0";
    let res = classify_traffic(Some(ua), None, None, None);

    assert_eq!(res.client_type, ClientType::AutomatedScript);
    assert_eq!(res.confidence, 99);
    assert!(res.browser_os.contains("Python Requests"));
}

#[test]
fn test_classify_googlebot() {
    let ua = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
    let res = classify_traffic(Some(ua), None, None, None);

    assert_eq!(res.client_type, ClientType::KnownBot);
    assert!(res.confidence >= 95);
    assert!(res.browser_os.contains("Googlebot"));
}

#[test]
fn test_classify_headless_chrome() {
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/120.0.0.0 Safari/537.36";
    let res = classify_traffic(Some(ua), None, None, None);

    assert_eq!(res.client_type, ClientType::AutomatedScript);
    assert_eq!(res.confidence, 99);
    assert!(res.browser_os.contains("Headless Chrome"));
}

#[test]
fn test_classify_empty_ua() {
    let res = classify_traffic(None, None, None, None);

    assert_eq!(res.client_type, ClientType::AutomatedScript);
    assert_eq!(res.confidence, 99);
}

#[test]
fn test_classify_suspicious_fake_ua() {
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)";
    let res = classify_traffic(Some(ua), None, None, None);

    assert_eq!(res.client_type, ClientType::Suspicious);
    assert!(res.confidence >= 70);
}
