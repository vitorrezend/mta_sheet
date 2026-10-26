//! Versões em brotli dos arquivos do front (WebAssembly, JS e CSS), comprimidas uma
//! única vez e guardadas em memória.
//!
//! Comprimir a WebAssembly (~7,5 MB) a cada download custaria CPU por visitante; aqui
//! ela é comprimida no início do servidor, com nível alto, e servida pronta. Enquanto o
//! cache aquece, a resposta sai sem compressão prévia (a camada dinâmica ainda comprime).

use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, LazyLock, RwLock};

/// Nível alto: o custo é pago uma vez só (o máximo, 11, levaria dezenas de segundos)
const QUALITY: u32 = 9;
const WINDOW_BITS: u32 = 22;

static CACHE: LazyLock<RwLock<HashMap<String, Arc<Vec<u8>>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

/// Arquivos que valem a pena guardar comprimidos.
pub fn is_compressible(key: &str) -> bool {
    key.ends_with(".wasm") || key.ends_with(".js") || key.ends_with(".css")
}

pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() / 4);
    {
        let mut writer = brotli::CompressorWriter::new(&mut out, 64 * 1024, QUALITY, WINDOW_BITS);
        if writer.write_all(data).is_err() {
            return Vec::new();
        }
    }
    out
}

/// Versão comprimida de `key`, se já estiver pronta.
pub fn get(key: &str) -> Option<Arc<Vec<u8>>> {
    CACHE.read().ok()?.get(key).cloned()
}

#[cfg_attr(debug_assertions, allow(dead_code))]
fn insert(key: &str, compressed: Vec<u8>) {
    if compressed.is_empty() {
        return;
    }
    if let Ok(mut cache) = CACHE.write() {
        cache.insert(key.to_string(), Arc::new(compressed));
    }
}

/// O navegador aceita brotli?
pub fn accepts_brotli(req: &axum::extract::Request) -> bool {
    req.headers()
        .get(http::header::ACCEPT_ENCODING)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.split(',').any(|enc| enc.trim().split(';').next() == Some("br")))
}

/// Comprime em segundo plano os arquivos embutidos do front, no início do servidor.
#[cfg(not(debug_assertions))]
pub fn warm_up() {
    tokio::task::spawn_blocking(|| {
        for key in super::static_files::SiteAssets::iter() {
            if !is_compressible(&key) || get(&key).is_some() {
                continue;
            }
            if let Some(file) = super::static_files::SiteAssets::get(&key) {
                let started = std::time::Instant::now();
                let compressed = compress(&file.data);
                tracing_log(&key, file.data.len(), compressed.len(), started.elapsed());
                insert(&key, compressed);
            }
        }
    });
}

#[cfg(not(debug_assertions))]
fn tracing_log(key: &str, original: usize, compressed: usize, took: std::time::Duration) {
    log::info!("brotli {}: {} KB -> {} KB em {:?}", key, original / 1024, compressed / 1024, took);
}

/// Em desenvolvimento os arquivos mudam a cada build: nada a pré-comprimir.
#[cfg(debug_assertions)]
pub fn warm_up() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn compressed_bytes_round_trip() {
        let data = "função main() { let x = 1; }\n".repeat(500).into_bytes();
        let compressed = compress(&data);
        assert!(compressed.len() < data.len() / 5);
        let mut decoded = Vec::new();
        brotli::Decompressor::new(compressed.as_slice(), 4096).read_to_end(&mut decoded).ok();
        assert_eq!(decoded, data);
    }

    #[test]
    fn only_front_assets_are_cached() {
        assert!(is_compressible("pkg/mta_sheet.wasm"));
        assert!(is_compressible("pkg/mta_sheet.css"));
        assert!(!is_compressible("banner_og.jpg"));
    }
}
