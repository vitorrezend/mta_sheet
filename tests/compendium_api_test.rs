#[cfg(feature = "ssr")]
mod tests {
    use axum::extract::Path;
    use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
    use mta_sheet::server::handlers::compendium_api_handler;

    #[tokio::test]
    async fn test_compendium_api_all_sections_return_200_and_cache_headers() {
        let sections = ["weapons", "practices", "instruments", "archetypes", "attributes"];

        for sec in sections {
            let res = compendium_api_handler(Path(sec.to_string()), HeaderMap::new()).await;
            assert_eq!(res.status(), StatusCode::OK, "Seção {} deve retornar 200 OK", sec);

            let headers = res.headers();
            assert_eq!(
                headers.get(header::CONTENT_TYPE).unwrap(),
                "application/json; charset=utf-8"
            );
            assert_eq!(
                headers.get(header::CACHE_CONTROL).unwrap(),
                "public, max-age=31536000, immutable"
            );
            assert!(headers.get(header::ETAG).is_some(), "Deve possuir cabeçalho ETag");
        }
    }

    #[tokio::test]
    async fn test_compendium_api_etag_304_not_modified() {
        let res = compendium_api_handler(Path("weapons".to_string()), HeaderMap::new()).await;
        let etag = res.headers().get(header::ETAG).unwrap().to_str().unwrap().to_string();

        let mut headers = HeaderMap::new();
        headers.insert(header::IF_NONE_MATCH, HeaderValue::from_str(&etag).unwrap());

        let res_cached = compendium_api_handler(Path("weapons".to_string()), headers).await;
        assert_eq!(
            res_cached.status(),
            StatusCode::NOT_MODIFIED,
            "Requisição com ETag coincidente deve retornar 304 Not Modified"
        );
    }

    #[tokio::test]
    async fn test_compendium_api_invalid_section_returns_404() {
        let res = compendium_api_handler(Path("inexistente".to_string()), HeaderMap::new()).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
