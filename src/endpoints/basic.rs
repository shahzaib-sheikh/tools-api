use std::collections::HashMap;
use rocket::figment::value::Map;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};

use crate::extractors::{AllHeaders, ClientIp};
use crate::types::{EchoResponse, IpInfoResponse, WhoamiResponse};

// Root endpoint
#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! { field: "value" })
}

// Basic endpoints
#[get("/whoami")]
pub fn whoami(whoami: WhoamiResponse) -> Json<WhoamiResponse> {
    Json(whoami)
}

#[get("/ip")]
pub fn ip(client_ip: ClientIp) -> String {
    client_ip.0
}

#[get("/ip-info")]
pub fn ip_info(client_ip: ClientIp) -> Json<IpInfoResponse> {
    // Simplified IP info without external API call
    Json(IpInfoResponse {
        ip: client_ip.0,
        city: Some("Unknown".to_string()),
        region: Some("Unknown".to_string()),
        country: Some("Unknown".to_string()),
        asn: Some("Unknown".to_string()),
        org: Some("Unknown".to_string()),
    })
}

#[get("/headers")]
pub fn headers(all_headers: AllHeaders) -> Json<Map<String, String>> {
    Json(all_headers.0)
}

#[get("/user-agent")]
pub fn user_agent(all_headers: AllHeaders) -> String {
    all_headers.0
        .get("user-agent")
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string())
}

#[get("/echo")]
pub fn echo(all_headers: AllHeaders) -> Json<EchoResponse> {
    let query_map: HashMap<String, String> = HashMap::new();
    
    Json(EchoResponse {
        method: "GET".to_string(),
        query: query_map,
        headers: all_headers.0,
        body: "".to_string(), // GET requests don't have body
    })
}

#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}
