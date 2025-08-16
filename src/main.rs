use std::collections::HashMap;
use std::net::IpAddr;
use std::process::Command;
use std::time::Duration;

use base64;
use chrono::Utc;
use md5;
use rand::Rng;
use rocket::figment::value::Map;
use rocket::request::{FromRequest, Request};
use rocket::response::status;
use rocket::serde::json::{Json, Value, serde_json};
use rocket::serde::Serialize;
use rocket::tokio::time::sleep;
use rocket_dyn_templates::{context, Template};
use sha1;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[macro_use]
extern crate rocket;



// Structs for responses
#[derive(Serialize)]
struct WhoamiResponse {
    ip: String,
    cookies: Map<String, String>,
    headers: Map<String, String>,
}

#[derive(Serialize)]
struct IpInfoResponse {
    ip: String,
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    asn: Option<String>,
    org: Option<String>,
}

#[derive(Serialize)]
struct EchoResponse {
    method: String,
    query: HashMap<String, String>,
    headers: Map<String, String>,
    body: String,
}

#[derive(Serialize)]
struct TimestampResponse {
    seconds: i64,
    milliseconds: i64,
}

#[derive(Serialize)]
struct TimeResponse {
    datetime: String,
    timezone: String,
}

#[derive(Serialize)]
struct JwtDecodeResponse {
    header: Value,
    payload: Value,
}

#[derive(Serialize)]
struct KeypairResponse {
    algorithm: String,
    public_key: String,
    private_key: String,
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

// Helper struct for getting client IP
#[derive(Debug)]
struct ClientIp(String);

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
struct AllHeaders(Map<String, String>);

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

// Root endpoint
#[get("/")]
fn index() -> Template {
    Template::render("index", context! { field: "value" })
}

// Basic endpoints
#[get("/whoami")]
fn whoami(whoami: WhoamiResponse) -> Json<WhoamiResponse> {
    Json(whoami)
}

#[get("/ip")]
fn ip(client_ip: ClientIp) -> String {
    client_ip.0
}

#[get("/ip-info")]
fn ip_info(client_ip: ClientIp) -> Json<IpInfoResponse> {
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
fn headers(all_headers: AllHeaders) -> Json<Map<String, String>> {
    Json(all_headers.0)
}

#[get("/user-agent")]
fn user_agent(all_headers: AllHeaders) -> String {
    all_headers.0
        .get("user-agent")
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string())
}

#[get("/echo")]
fn echo(all_headers: AllHeaders) -> Json<EchoResponse> {
    let query_map: HashMap<String, String> = HashMap::new();
    
    Json(EchoResponse {
        method: "GET".to_string(),
        query: query_map,
        headers: all_headers.0,
        body: "".to_string(), // GET requests don't have body
    })
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

// Network utilities
#[get("/trace/<host>")]
async fn trace(host: String) -> String {
    let output = Command::new("traceroute")
        .arg(&host)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                String::from_utf8_lossy(&result.stdout).to_string()
            } else {
                format!("Error: {}", String::from_utf8_lossy(&result.stderr))
            }
        }
        Err(e) => format!("Failed to execute traceroute: {}", e),
    }
}

#[get("/dns/<domain>")]
async fn dns_lookup(domain: String) -> Json<Value> {
    // Simple DNS lookup using nslookup command
    let output = Command::new("nslookup")
        .arg(&domain)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                let output_str = String::from_utf8_lossy(&result.stdout);
                Json(serde_json::json!({
                    "domain": domain,
                    "result": output_str.to_string()
                }))
            } else {
                Json(serde_json::json!({
                    "domain": domain,
                    "error": String::from_utf8_lossy(&result.stderr).to_string()
                }))
            }
        }
        Err(e) => Json(serde_json::json!({
            "domain": domain,
            "error": format!("Failed to execute nslookup: {}", e)
        }))
    }
}

// Utility endpoints
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> &'static str {
    sleep(Duration::from_secs(seconds)).await;
    "OK"
}

#[get("/status/<code>")]
fn status_code(code: u16) -> status::Custom<&'static str> {
    status::Custom(rocket::http::Status::from_code(code).unwrap_or(rocket::http::Status::Ok), "")
}

#[get("/uuid")]
fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

// Generator endpoints
#[get("/lorem/<words>")]
fn lorem(words: usize) -> String {
    let lorem_words = vec![
        "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit",
        "sed", "do", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore",
        "magna", "aliqua", "enim", "ad", "minim", "veniam", "quis", "nostrud",
        "exercitation", "ullamco", "laboris", "nisi", "aliquip", "ex", "ea", "commodo",
        "consequat", "duis", "aute", "irure", "in", "reprehenderit", "voluptate",
        "velit", "esse", "cillum", "fugiat", "nulla", "pariatur", "excepteur", "sint",
        "occaecat", "cupidatat", "non", "proident", "sunt", "culpa", "qui", "officia",
        "deserunt", "mollit", "anim", "id", "est", "laborum"
    ];
    
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    
    for _ in 0..words {
        let word_index = rng.gen_range(0..lorem_words.len());
        result.push(lorem_words[word_index]);
    }
    
    result.join(" ")
}

#[get("/color")]
fn random_color() -> String {
    let mut rng = rand::thread_rng();
    format!("#{:06x}", rng.gen::<u32>() & 0xFFFFFF)
}

#[get("/password/<length>")]
fn generate_password(length: usize) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

#[get("/number/<min>/<max>")]
fn random_number(min: i32, max: i32) -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max).to_string()
}

// Encoding endpoints
#[get("/base64/<text>")]
fn base64_encode(text: String) -> String {
    base64::encode(text.as_bytes())
}

#[get("/base64-decode/<b64>")]
fn base64_decode(b64: String) -> String {
    match base64::decode(&b64) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => "Invalid base64".to_string(),
    }
}

#[get("/urlencode/<text>")]
fn url_encode(text: String) -> String {
    // Simple URL encoding implementation
    text.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "%20".to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

#[get("/urldecode/<encoded>")]
fn url_decode(encoded: String) -> String {
    // Simple URL decoding implementation
    let mut result = String::new();
    let mut chars = encoded.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            if let (Some(h1), Some(h2)) = (chars.next(), chars.next()) {
                if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                    result.push(byte as char);
                } else {
                    result.push(c);
                    result.push(h1);
                    result.push(h2);
                }
            } else {
                result.push(c);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    
    result
}

// Crypto endpoints
#[get("/hash/<algo>/<text>")]
fn hash(algo: String, text: String) -> String {
    match algo.to_lowercase().as_str() {
        "md5" => {
            format!("{:x}", md5::compute(text.as_bytes()))
        }
        "sha1" => {
            use sha1::{Digest, Sha1};
            let mut hasher = Sha1::new();
            hasher.update(text.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(text.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        _ => "Unsupported algorithm".to_string(),
    }
}

// Time endpoints
#[get("/timestamp")]
fn timestamp() -> Json<TimestampResponse> {
    let now = Utc::now();
    Json(TimestampResponse {
        seconds: now.timestamp(),
        milliseconds: now.timestamp_millis(),
    })
}

#[get("/time/utc")]
fn time_utc() -> Json<TimeResponse> {
    let now = Utc::now();
    Json(TimeResponse {
        datetime: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        timezone: "UTC".to_string(),
    })
}

#[get("/time/<tz>")]
fn time_tz(tz: String) -> Json<TimeResponse> {
    let now = Utc::now();
    
    // For simplicity, we'll just return UTC time with the requested timezone name
    // In a production app, you'd want to use a proper timezone library
    Json(TimeResponse {
        datetime: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        timezone: tz,
    })
}

// JWT decode endpoint
#[get("/jwt-decode/<token>")]
fn jwt_decode(token: String) -> Json<Value> {
    // Decode without verification (for demonstration purposes)
    let parts: Vec<&str> = token.split('.').collect();
    
    if parts.len() != 3 {
        return Json(serde_json::json!({"error": "Invalid JWT format"}));
    }
    
    let decode_part = |part: &str| -> Result<Value, Box<dyn std::error::Error>> {
        // Add padding if needed
        let mut padded = part.to_string();
        while padded.len() % 4 != 0 {
            padded.push('=');
        }
        
        let decoded = base64::decode(&padded.replace('-', "+").replace('_', "/"))?;
        let json: Value = serde_json::from_slice(&decoded)?;
        Ok(json)
    };
    
    match (decode_part(parts[0]), decode_part(parts[1])) {
        (Ok(header), Ok(payload)) => {
            Json(serde_json::json!({
                "header": header,
                "payload": payload
            }))
        }
        _ => Json(serde_json::json!({"error": "Failed to decode JWT"})),
    }
}

// Keypair generation endpoint
#[get("/generate-keypair/<algo>")]
fn generate_keypair(algo: String) -> Json<Value> {
    match algo.to_lowercase().as_str() {
        "rsa" => {
            Json(serde_json::json!({
                "algorithm": "RSA",
                "private_key": "RSA private key (simplified implementation)",
                "public_key": "RSA public key (simplified implementation)",
                "note": "Full RSA implementation requires additional dependencies"
            }))
        }
        "ed25519" => {
            // Generate random bytes for demonstration
            let mut rng = rand::thread_rng();
            let private_key: [u8; 32] = (0..32).map(|_| rng.gen()).collect::<Vec<u8>>().try_into().unwrap();
            let public_key: [u8; 32] = (0..32).map(|_| rng.gen()).collect::<Vec<u8>>().try_into().unwrap();
            
            Json(serde_json::json!({
                "algorithm": "Ed25519",
                "private_key": hex::encode(private_key),
                "public_key": hex::encode(public_key),
                "note": "Simplified implementation for demonstration"
            }))
        }
        "secp256k1" => {
            // Generate random bytes for demonstration
            let mut rng = rand::thread_rng();
            let private_key: [u8; 32] = (0..32).map(|_| rng.gen()).collect::<Vec<u8>>().try_into().unwrap();
            let public_key: [u8; 33] = (0..33).map(|_| rng.gen()).collect::<Vec<u8>>().try_into().unwrap();
            
            Json(serde_json::json!({
                "algorithm": "secp256k1",
                "private_key": hex::encode(private_key),
                "public_key": hex::encode(public_key),
                "note": "Simplified implementation for demonstration"
            }))
        }
        _ => Json(serde_json::json!({"error": "Unsupported algorithm. Supported: rsa, ed25519, secp256k1"})),
    }
}

// Fun endpoints
#[get("/cat-fact")]
fn cat_fact() -> String {
    let facts = vec![
        "Cats sleep 12-16 hours a day.",
        "A group of cats is called a 'clowder'.",
        "Cats have over 30 muscles controlling their ears.",
        "A cat's purr vibrates at a frequency that can help heal bones.",
        "Cats can rotate their ears 180 degrees.",
    ];
    
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..facts.len());
    facts[index].to_string()
}

#[get("/quote")]
fn quote() -> String {
    let quotes = vec![
        "The only way to do great work is to love what you do. - Steve Jobs",
        "Innovation distinguishes between a leader and a follower. - Steve Jobs",
        "Life is what happens to you while you're busy making other plans. - John Lennon",
        "The future belongs to those who believe in the beauty of their dreams. - Eleanor Roosevelt",
        "It is during our darkest moments that we must focus to see the light. - Aristotle",
    ];
    
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..quotes.len());
    quotes[index].to_string()
}

pub fn create_rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![
            index,
            whoami,
            ip,
            ip_info,
            headers,
            user_agent,
            echo,
            ping,
            trace,
            dns_lookup,
            delay,
            status_code,
            generate_uuid,
            lorem,
            random_color,
            generate_password,
            random_number,
            base64_encode,
            base64_decode,
            url_encode,
            url_decode,
            hash,
            timestamp,
            time_utc,
            time_tz,
            jwt_decode,
            generate_keypair,
            cat_fact,
            quote
        ])
}

#[launch]
fn rocket() -> _ {
    create_rocket()
}