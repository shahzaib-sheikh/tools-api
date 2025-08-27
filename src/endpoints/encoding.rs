use base64;

// Encoding endpoints - FIXED: Updated to work with base64 v0.13
#[get("/base64/<text>")]
pub fn base64_encode(text: String) -> String {
    base64::encode(text.as_bytes())
}

#[get("/base64-decode/<b64>")]
pub fn base64_decode(b64: String) -> String {
    match base64::decode(&b64) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => "Error: Invalid base64 encoding".to_string(),
    }
}

#[get("/urlencode/<text>")]
pub fn url_encode(text: String) -> String {
    // Proper URL encoding that handles Unicode correctly
    text.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            b' ' => "%20".to_string(),
            _ => format!("%{:02X}", b),
        })
        .collect()
}

#[get("/urldecode/<encoded>")]
pub fn url_decode(encoded: String) -> String {
    // SECURITY FIX: Safe URL decoding with proper UTF-8 validation
    let mut bytes = Vec::new();
    let mut chars = encoded.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            if let (Some(h1), Some(h2)) = (chars.next(), chars.next()) {
                if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                    bytes.push(byte);
                } else {
                    // Invalid hex sequence, treat as literal characters
                    bytes.extend_from_slice(c.to_string().as_bytes());
                    bytes.extend_from_slice(h1.to_string().as_bytes());
                    bytes.extend_from_slice(h2.to_string().as_bytes());
                }
            } else {
                // Incomplete percent encoding, treat as literal
                bytes.extend_from_slice(c.to_string().as_bytes());
            }
        } else if c == '+' {
            bytes.push(b' ');
        } else {
            bytes.extend_from_slice(c.to_string().as_bytes());
        }
    }
    
    // SECURITY FIX: Use from_utf8_lossy for safe UTF-8 conversion
    String::from_utf8_lossy(&bytes).to_string()
}
