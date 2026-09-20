use axum::response::IntoResponse;
use http::header::{CACHE_CONTROL, CONTENT_TYPE};
use http::StatusCode;
use sqlx::Row;

pub const ROBOTS_TXT_CONTENT: &str = "\
User-agent: *
Allow: /
Disallow: /api/
Disallow: /uploads/

Sitemap: /sitemap.xml
";

pub const FAVICON_SVG_CONTENT: &str = r###"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64">
  <defs>
    <radialGradient id="orbGrad" cx="35%" cy="30%" r="70%">
      <stop offset="0%" stop-color="#c084fc"/>
      <stop offset="45%" stop-color="#7e22ce"/>
      <stop offset="85%" stop-color="#2e1065"/>
      <stop offset="100%" stop-color="#0f051d"/>
    </radialGradient>
    <linearGradient id="goldRing" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#ffd700"/>
      <stop offset="50%" stop-color="#caa75d"/>
      <stop offset="100%" stop-color="#7a5b20"/>
    </linearGradient>
    <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
      <feGaussianBlur stdDeviation="2" result="blur"/>
      <feComposite in="SourceGraphic" in2="blur" operator="over"/>
    </filter>
  </defs>
  <!-- Background Ring -->
  <circle cx="32" cy="32" r="30" fill="none" stroke="url(#goldRing)" stroke-width="2.5" opacity="0.9"/>
  <!-- Mystic Orb -->
  <circle cx="32" cy="32" r="24" fill="url(#orbGrad)" filter="url(#glow)"/>
  <!-- Pentagram / Arcane Star Core -->
  <path d="M32 14 L36.5 25.5 L49 26 L39 34 L43 46 L32 39 L21 46 L25 34 L15 26 L27.5 25.5 Z" fill="url(#goldRing)" opacity="0.92"/>
  <!-- Center Light Sparkle -->
  <circle cx="28" cy="24" r="2.5" fill="#ffffff" opacity="0.8"/>
  <circle cx="23" cy="28" r="1.2" fill="#ffffff" opacity="0.5"/>
</svg>"###;

pub async fn robots_txt_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (CONTENT_TYPE, "text/plain; charset=utf-8"),
            (CACHE_CONTROL, "public, max-age=86400"),
        ],
        ROBOTS_TXT_CONTENT,
    )
}

/// Handler dinâmico de Sitemap que inclui rotas canônicas fixas e todas as fichas públicas
/// da comunidade criadas pelos jogadores (para indexação orgânica no Google).
pub async fn sitemap_xml_handler() -> impl IntoResponse {
    let pool = crate::database::get_db().await;

    // Busca as fichas públicas da comunidade para inclusão no sitemap
    let sheets: Vec<(String, String)> = sqlx::query(
        "SELECT id, COALESCE(strftime('%Y-%m-%d', updated_at), strftime('%Y-%m-%d', 'now')) as lastmod FROM character_sheets WHERE is_public = 1 ORDER BY updated_at DESC LIMIT 1000"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|r| (r.get::<String, _>("id"), r.get::<String, _>("lastmod")))
    .collect();

    // Busca os perfis de usuários com fichas públicas
    let users: Vec<String> = sqlx::query(
        "SELECT DISTINCT u.username FROM users u JOIN character_sheets cs ON u.id = cs.user_id WHERE cs.is_public = 1 LIMIT 200"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|r| r.get::<String, _>("username"))
    .collect();

    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>/</loc>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>/feed</loc>
    <changefreq>daily</changefreq>
    <priority>0.9</priority>
  </url>
  <url>
    <loc>/rooms</loc>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>/about</loc>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>/login</loc>
    <changefreq>monthly</changefreq>
    <priority>0.5</priority>
  </url>
"#);

    for (id, lastmod) in sheets {
        let lastmod_tag = if !lastmod.is_empty() {
            format!("\n    <lastmod>{}</lastmod>", lastmod)
        } else {
            String::new()
        };
        xml.push_str(&format!(
            "  <url>\n    <loc>/sheet/{}</loc>{}\n    <changefreq>weekly</changefreq>\n    <priority>0.7</priority>\n  </url>\n",
            id, lastmod_tag
        ));
    }

    for username in users {
        xml.push_str(&format!(
            "  <url>\n    <loc>/user/{}</loc>\n    <changefreq>weekly</changefreq>\n    <priority>0.6</priority>\n  </url>\n",
            username
        ));
    }

    xml.push_str("</urlset>\n");

    (
        StatusCode::OK,
        [
            (CONTENT_TYPE, "application/xml; charset=utf-8"),
            (CACHE_CONTROL, "public, max-age=3600, stale-while-revalidate=600"),
        ],
        xml,
    )
}

pub async fn favicon_svg_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (CONTENT_TYPE, "image/svg+xml"),
            (CACHE_CONTROL, "public, max-age=604800"),
        ],
        FAVICON_SVG_CONTENT,
    )
}

pub async fn favicon_ico_handler() -> impl IntoResponse {
    favicon_svg_handler().await
}

/// Serve o banner oficial de alta resolução Open Graph (16:9) para prévias ricas
/// em redes sociais (WhatsApp, Discord, Twitter/X, Facebook, Telegram).
pub async fn banner_og_handler() -> impl IntoResponse {
    if let Ok(bytes) = tokio::fs::read("data/banner_og.jpg").await {
        return (
            StatusCode::OK,
            [
                (CONTENT_TYPE, "image/jpeg"),
                (CACHE_CONTROL, "public, max-age=604800, stale-while-revalidate=86400"),
            ],
            bytes,
        )
            .into_response();
    }
    if let Ok(bytes) = tokio::fs::read("target/site/banner_og.jpg").await {
        return (
            StatusCode::OK,
            [
                (CONTENT_TYPE, "image/jpeg"),
                (CACHE_CONTROL, "public, max-age=604800, stale-while-revalidate=86400"),
            ],
            bytes,
        )
            .into_response();
    }
    favicon_svg_handler().await.into_response()
}
