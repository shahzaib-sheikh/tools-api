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
        
        let client_ip = request
            .headers()
            .get_one("x-real-ip")
            .or_else(|| request.headers().get_one("x-forwarded-for"))
            .map_or(default_ip.to_string(), |ip| {
                ip.split(',').next().unwrap_or(ip).trim().to_string()
            });
        
        rocket::request::Outcome::Success(ClientIp(client_ip))
    }
}

// Helper struct for getting all headers
#[derive(Debug)]
pub struct AllHeaders(pub Map<String, String>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AllHeaders {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let mut headers_map: Map<String, String> = Map::new();
        
        for h in request.headers().iter() {
            headers_map.insert(h.name().to_string(), h.value().to_string());
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

        let remote_ip: IpAddr = request
            .headers()
            .get_one("x-real-ip")
            .or_else(|| request.headers().get_one("x-forwarded-for"))
            .map_or(default_ip, |ip| {
                ip.split(',').next().unwrap_or(ip).trim().parse().unwrap_or(default_ip)
            });

        rocket::request::Outcome::Success(WhoamiResponse {
            ip: remote_ip.to_string(),
            cookies: cookie_map,
            headers,
        })
    }
}
