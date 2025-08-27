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
        
        // Prioritize x-real-ip header, then fall back to other methods
        let client_ip = request
            .headers()
            .get_one("x-real-ip")
            .and_then(|ip| {
                // Validate it's a proper IP address
                ip.trim().parse::<IpAddr>().ok()?.to_string().into()
            })
            .or_else(|| {
                // Fall back to Rocket's built-in remote address when available
                request.remote().map(|addr| addr.ip().to_string())
            })
            .or_else(|| {
                // Finally fall back to x-forwarded-for header
                request
                    .headers()
                    .get_one("x-forwarded-for")
                    .and_then(|ip| {
                        // Take only the first IP from comma-separated list
                        let first_ip = ip.split(',').next()?.trim();
                        // Validate it's a proper IP address
                        first_ip.parse::<IpAddr>().ok()?.to_string().into()
                    })
            })
            .unwrap_or_else(|| default_ip.to_string());
        
        rocket::request::Outcome::Success(ClientIp(client_ip))
    }
}

// Helper struct for getting all headers (used by general endpoints)
#[derive(Debug)]
pub struct AllHeaders(pub Map<String, String>);

// List of sensitive headers that should not be exposed in general endpoints
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
            // SECURITY: Filter out sensitive headers and proxy/infrastructure headers for general endpoints
            if !SENSITIVE_HEADERS.contains(&header_name.as_str()) 
                && !header_name.starts_with("x-")
                && !header_name.starts_with("cf-")
                && !header_name.starts_with("cdn-") {
                headers_map.insert(h.name().to_string(), h.value().to_string());
            }
        }
        
        rocket::request::Outcome::Success(AllHeaders(headers_map))
    }
}

// FromRequest implementation for WhoamiResponse (debugging endpoint - shows most headers)
#[rocket::async_trait]
impl<'r> FromRequest<'r> for WhoamiResponse {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        // Extract country from cf-ipcountry header before filtering
        let country = request
            .headers()
            .get_one("cf-ipcountry")
            .map(|country| country.to_string());

        let mut headers: Map<String, String> = Map::new();

        for h in request.headers().iter() {
            let header_name = h.name().as_str().to_lowercase();
            // SECURITY FIX: Filter out proxy/infrastructure headers for debugging clarity
            // Keep standard headers visible for debugging purposes
            if !header_name.starts_with("x-")
                && !header_name.starts_with("cf-")
                && !header_name.starts_with("cdn-") {
                headers.insert(h.name().to_string(), h.value().to_string());
            }
        }

        let mut cookie_map: Map<String, String> = Map::new();

        for c in request.cookies().iter() {
            cookie_map.insert(c.name().to_string(), c.value().to_string());
        }

        let default_ip: IpAddr = "127.0.0.1".parse().unwrap();

        // Use same IP extraction logic as ClientIp - prioritize x-real-ip
        let remote_ip = request
            .headers()
            .get_one("x-real-ip")
            .and_then(|ip| {
                // Validate it's a proper IP address
                ip.trim().parse::<IpAddr>().ok()
            })
            .or_else(|| {
                // Fall back to Rocket's built-in remote address when available
                request.remote().map(|addr| addr.ip())
            })
            .or_else(|| {
                // Finally fall back to x-forwarded-for header
                request
                    .headers()
                    .get_one("x-forwarded-for")
                    .and_then(|ip| {
                        // Take only the first IP from comma-separated list
                        let first_ip = ip.split(',').next()?.trim();
                        // Validate it's a proper IP address
                        first_ip.parse::<IpAddr>().ok()
                    })
            })
            .unwrap_or(default_ip);

        rocket::request::Outcome::Success(WhoamiResponse {
            ip: remote_ip.to_string(),
            country,
            cookies: cookie_map,
            headers,
        })
    }
}
