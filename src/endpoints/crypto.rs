use base64;
use md5;
use sha1;
use sha2::{Digest, Sha256};
use rocket::serde::json::{Json, Value, serde_json};

// Crypto endpoints
#[get("/hash/<algo>/<text>")]
pub fn hash(algo: String, text: String) -> String {
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
        _ => "Error: Unsupported hash algorithm (supported: md5, sha1, sha256)".to_string(),
    }
}

// JWT decode endpoint
#[get("/jwt-decode/<token>")]
pub fn jwt_decode(token: String) -> Json<Value> {
    // SECURITY WARNING: This endpoint decodes JWT tokens for inspection only.
    // It does NOT verify signatures or validate claims. Never use this for authentication!
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
                "warning": "This is for inspection only - signature NOT verified!",
                "header": header,
                "payload": payload
            }))
        }
        _ => Json(serde_json::json!({"error": "Invalid JWT format or encoding"})),
    }
}
