//! Resolução do IP real do cliente.
//!
//! Cabeçalhos `X-Forwarded-For` / `X-Real-IP` só são considerados quando a conexão TCP vem
//! de um proxy confiável (variável `TRUSTED_PROXIES`, lista de IPs/CIDRs separada por vírgula).
//! Sem a variável, apenas loopback é confiável (ex.: Caddy/Nginx rodando na mesma máquina).
//! Assim, um cliente direto não consegue forjar o próprio IP para escapar do rate limit.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::LazyLock;

/// IP do cliente já resolvido, inserido nas extensões da requisição pelo [`client_ip_middleware`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClientIp(pub IpAddr);

/// Faixa de endereços no formato CIDR (`10.0.0.0/8`, `::1/128`) ou IP único.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IpNet {
    addr: IpAddr,
    prefix: u8,
}

impl IpNet {
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        let (addr_str, prefix_str) = match s.split_once('/') {
            Some((a, p)) => (a, Some(p)),
            None => (s, None),
        };
        let addr: IpAddr = addr_str.trim().parse().ok()?;
        let addr = addr.to_canonical();
        let max = if addr.is_ipv4() { 32 } else { 128 };
        let prefix = match prefix_str {
            Some(p) => p.trim().parse::<u8>().ok().filter(|p| *p <= max)?,
            None => max,
        };
        Some(Self { addr, prefix })
    }

    pub fn contains(&self, ip: IpAddr) -> bool {
        match (self.addr, ip.to_canonical()) {
            (IpAddr::V4(net), IpAddr::V4(ip)) => {
                let mask = u32::MAX.checked_shl(32 - u32::from(self.prefix)).unwrap_or(0);
                u32::from(net) & mask == u32::from(ip) & mask
            }
            (IpAddr::V6(net), IpAddr::V6(ip)) => {
                let mask = u128::MAX.checked_shl(128 - u32::from(self.prefix)).unwrap_or(0);
                u128::from(net) & mask == u128::from(ip) & mask
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TrustedProxies(Vec<IpNet>);

impl TrustedProxies {
    /// `None` (variável ausente) => apenas loopback. Entradas inválidas são ignoradas com aviso.
    pub fn from_config(value: Option<&str>) -> Self {
        let Some(value) = value else {
            return Self(vec![
                IpNet { addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)), prefix: 8 },
                IpNet { addr: IpAddr::V6(Ipv6Addr::LOCALHOST), prefix: 128 },
            ]);
        };
        let nets = value
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|entry| {
                let parsed = IpNet::parse(entry);
                if parsed.is_none() {
                    log::warn!("TRUSTED_PROXIES: entrada inválida ignorada: '{}'", entry);
                }
                parsed
            })
            .collect();
        Self(nets)
    }

    pub fn is_trusted(&self, ip: IpAddr) -> bool {
        self.0.iter().any(|net| net.contains(ip))
    }
}

static TRUSTED_PROXIES: LazyLock<TrustedProxies> =
    LazyLock::new(|| TrustedProxies::from_config(std::env::var("TRUSTED_PROXIES").ok().as_deref()));

fn parse_forwarded_ip(raw: &str) -> Option<IpAddr> {
    let raw = raw.trim();
    raw.parse::<IpAddr>()
        .or_else(|_| raw.parse::<SocketAddr>().map(|s| s.ip()))
        .ok()
        .map(|ip| ip.to_canonical())
}

/// Determina o IP real do cliente a partir do par TCP (`peer`) e dos cabeçalhos de proxy.
///
/// A cadeia do `X-Forwarded-For` é lida da direita para a esquerda (cada proxy acrescenta
/// ao final), parando no primeiro endereço que não é proxy confiável. Entradas à esquerda
/// dele foram escritas pelo próprio cliente e não são confiáveis.
pub fn resolve_client_ip(peer: IpAddr, headers: &http::HeaderMap, trusted: &TrustedProxies) -> IpAddr {
    let peer = peer.to_canonical();
    if !trusted.is_trusted(peer) {
        return peer;
    }

    let chain: Vec<IpAddr> = headers
        .get_all("x-forwarded-for")
        .iter()
        .filter_map(|h| h.to_str().ok())
        .flat_map(|h| h.split(','))
        .filter_map(parse_forwarded_ip)
        .collect();

    if let Some(ip) = chain.iter().rev().find(|ip| !trusted.is_trusted(**ip)) {
        return *ip;
    }
    if let Some(first) = chain.first() {
        return *first;
    }
    headers
        .get("x-real-ip")
        .and_then(|h| h.to_str().ok())
        .and_then(parse_forwarded_ip)
        .unwrap_or(peer)
}

/// Middleware que resolve o IP do cliente uma única vez e o disponibiliza como [`ClientIp`].
/// Deve ser a camada mais externa, antes de rate limit e logs.
pub async fn client_ip_middleware(
    mut request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    // Sem ConnectInfo (ex.: testes com `oneshot`) não há par TCP real: trata como loopback.
    let peer = request
        .extensions()
        .get::<axum::extract::ConnectInfo<SocketAddr>>()
        .map(|c| c.0.ip())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let ip = resolve_client_ip(peer, request.headers(), &TRUSTED_PROXIES);
    request.extensions_mut().insert(ClientIp(ip));
    next.run(request).await
}

/// IP resolvido em formato texto (chave do rate limit e dos logs).
/// Sem o middleware, todos caem no mesmo balde "unknown" em vez de ganharem isenção.
pub fn client_ip_from_extensions(extensions: &http::Extensions) -> String {
    extensions
        .get::<ClientIp>()
        .map(|c| c.0.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(pairs: &[(&str, &str)]) -> http::HeaderMap {
        let mut h = http::HeaderMap::new();
        for (k, v) in pairs {
            h.append(
                http::HeaderName::from_bytes(k.as_bytes()).expect("header name"),
                http::HeaderValue::from_str(v).expect("header value"),
            );
        }
        h
    }

    fn ip(s: &str) -> IpAddr {
        s.parse().expect("ip")
    }

    #[test]
    fn cidr_matching() {
        let net = IpNet::parse("172.16.0.0/12").expect("cidr");
        assert!(net.contains(ip("172.20.1.5")));
        assert!(!net.contains(ip("172.32.0.1")));
        assert!(net.contains(ip("::ffff:172.20.1.5")));
        assert!(IpNet::parse("0.0.0.0/0").expect("any").contains(ip("8.8.8.8")));
        assert!(IpNet::parse("10.0.0.0/33").is_none());
        assert!(IpNet::parse("lixo").is_none());
    }

    #[test]
    fn untrusted_peer_cannot_spoof_headers() {
        let trusted = TrustedProxies::from_config(None);
        let h = headers(&[("x-forwarded-for", "1.2.3.4"), ("x-real-ip", "5.6.7.8"), ("host", "localhost")]);
        assert_eq!(resolve_client_ip(ip("203.0.113.9"), &h, &trusted), ip("203.0.113.9"));
    }

    #[test]
    fn trusted_proxy_uses_rightmost_untrusted_hop() {
        let trusted = TrustedProxies::from_config(Some("172.16.0.0/12"));
        // Cliente tentou forjar "1.1.1.1"; o proxy acrescentou o IP real ao final
        let h = headers(&[("x-forwarded-for", "1.1.1.1, 198.51.100.7")]);
        assert_eq!(resolve_client_ip(ip("172.18.0.3"), &h, &trusted), ip("198.51.100.7"));
    }

    #[test]
    fn trusted_proxy_without_headers_falls_back_to_peer() {
        let trusted = TrustedProxies::from_config(None);
        assert_eq!(resolve_client_ip(ip("127.0.0.1"), &http::HeaderMap::new(), &trusted), ip("127.0.0.1"));
        let h = headers(&[("x-real-ip", "198.51.100.8")]);
        assert_eq!(resolve_client_ip(ip("127.0.0.1"), &h, &trusted), ip("198.51.100.8"));
    }

    #[test]
    fn empty_config_trusts_nobody() {
        let trusted = TrustedProxies::from_config(Some(""));
        let h = headers(&[("x-forwarded-for", "1.2.3.4")]);
        assert_eq!(resolve_client_ip(ip("127.0.0.1"), &h, &trusted), ip("127.0.0.1"));
    }
}
