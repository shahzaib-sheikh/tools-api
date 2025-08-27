use std::collections::HashMap;
use rocket::figment::value::Map;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct WhoamiResponse {
    pub ip: String,
    pub country: Option<String>,
    pub cookies: Map<String, String>,
    pub headers: Map<String, String>,
}

#[derive(Serialize)]
pub struct IpInfoResponse {
    pub ip: String,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    pub asn: Option<String>,
    pub org: Option<String>,
}

#[derive(Serialize)]
pub struct EchoResponse {
    pub method: String,
    pub query: HashMap<String, String>,
    pub headers: Map<String, String>,
    pub body: String,
}

#[derive(Serialize)]
pub struct TimestampResponse {
    pub seconds: i64,
    pub milliseconds: i64,
}

#[derive(Serialize)]
pub struct TimeResponse {
    pub datetime: String,
    pub timezone: String,
}
