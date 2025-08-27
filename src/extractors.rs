use std::net::IpAddr;
use rocket::figment::value::Map;
use rocket::request::{FromRequest, Request};
use crate::types::WhoamiResponse;

// Helper struct for getting client IP
#[derive(Debug)]
pub struct ClientIp(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientIp {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let default_ip: IpAddr = "127.0.0.1".parse().unwrap();
        
        // SECURITY FIX: Use Rocket's built-in remote address when available
        // Only fall back to headers if behind a trusted proxy
        let client_ip = if let Some(remote_addr) = request.remote() {
            remote_addr.ip().to_string()
        } else {
            // Validate and sanitize proxy headers
            let header_ip = request
                .headers()
                .get_one("x-forwarded-for")
                .and_then(|ip| {
                    // Take only the first IP from comma-separated list
                    let first_ip = ip.split(',').next()?.trim();
                    // Validate it's a proper IP address
                    first_ip.parse::<IpAddr>().ok()?.to_string().into()
                })
                .unwrap_or_else(|| default_ip.to_string());
            header_ip
        };
        
        rocket::request::Outcome::Success(ClientIp(client_ip))
    }
}

// Helper struct for getting all headers
#[derive(Debug)]
pub struct AllHeaders(pub Map<String, String>);

// List of sensitive headers that should not be exposed
const SENSITIVE_HEADERS: &[&str] = &[
    "authorization",
    "cookie",
    "set-cookie",
    "x-api-key",
    "x-auth-token",
    "x-access-token",
    "proxy-authorization",
    "www-authenticate",
];

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AllHeaders {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let mut headers_map: Map<String, String> = Map::new();
        
        for h in request.headers().iter() {
            let header_name = h.name().as_str().to_lowercase();
            // SECURITY FIX: Filter out sensitive headers
            if !SENSITIVE_HEADERS.contains(&header_name.as_str()) {
                headers_map.insert(h.name().to_string(), h.value().to_string());
            }
        }
        
        rocket::request::Outcome::Success(AllHeaders(headers_map))
    }
}

// FromRequest implementation for WhoamiResponse
#[rocket::async_trait]
impl<'r> FromRequest<'r> for WhoamiResponse {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let mut headers: Map<String, String> = Map::new();

        for h in request.headers().iter() {
            let header_name = h.name().to_string();
            headers.insert(header_name, h.value().to_string());
        }

        let mut cookie_map: Map<String, String> = Map::new();

        for c in request.cookies().iter() {
            cookie_map.insert(c.name().to_string(), c.value().to_string());
        }

        let default_ip: IpAddr = "127.0.0.1".parse().unwrap();

        // SECURITY FIX: Use same secure IP extraction logic
        let remote_ip = if let Some(remote_addr) = request.remote() {
            remote_addr.ip()
        } else {
            // Validate and sanitize proxy headers
            request
                .headers()
                .get_one("x-forwarded-for")
                .and_then(|ip| {
                    // Take only the first IP from comma-separated list
                    let first_ip = ip.split(',').next()?.trim();
                    // Validate it's a proper IP address
                    first_ip.parse::<IpAddr>().ok()
                })
                .unwrap_or(default_ip)
        };

        rocket::request::Outcome::Success(WhoamiResponse {
            ip: remote_ip.to_string(),
            cookies: cookie_map,
            headers,
        })
    }
}
