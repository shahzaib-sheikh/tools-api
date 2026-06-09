use base64::{Engine as _, engine::general_purpose};

// Encoding endpoints - SECURITY FIX: Updated to current base64 API v0.21
#[get("/base64/<text>")]
pub fn base64_encode(text: String) -> String {
    // SECURITY FIX: Limit input size to prevent DoS attacks
    if text.len() > 1_000_000 {
        return "Error: Input too large (max 1MB)".to_string();
    }
    general_purpose::STANDARD.encode(text.as_bytes())
}

#[get("/base64-decode/<b64>")]
pub fn base64_decode(b64: String) -> String {
    // SECURITY FIX: Limit input size to prevent DoS attacks
    if b64.len() > 1_500_000 {
        return "Error: Input too large (max ~1.5MB base64)".to_string();
    }
    match general_purpose::STANDARD.decode(&b64) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => "Error: Invalid base64 encoding".to_string(),
    }
}

#[get("/urlencode/<text>")]
pub fn url_encode(text: String) -> String {
    // SECURITY FIX: Limit input size to prevent DoS attacks
    if text.len() > 100_000 {
        return "Error: Input too large (max 100KB)".to_string();
    }
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
    // SECURITY FIX: Limit input size to prevent DoS attacks
    if encoded.len() > 100_000 {
        return "Error: Input too large (max 100KB)".to_string();
    }
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

// ---- Additional encoders (query-param based) ----

const MAX_ENC: usize = 100_000;

/// `/hex/encode?text=hi` -> "6869"
#[get("/hex/encode?<text>")]
pub fn hex_encode(text: String) -> String {
    if text.len() > MAX_ENC {
        return "Error: input too large (max 100KB)".to_string();
    }
    hex::encode(text.as_bytes())
}

/// `/hex/decode?input=6869` -> "hi"
#[get("/hex/decode?<input>")]
pub fn hex_decode(input: String) -> String {
    if input.len() > 2 * MAX_ENC {
        return "Error: input too large".to_string();
    }
    match hex::decode(input.trim()) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => "Error: invalid hex".to_string(),
    }
}

/// `/rot13?text=Hello` -> "Uryyb" (self-inverse)
#[get("/rot13?<text>")]
pub fn rot13(text: String) -> String {
    if text.len() > MAX_ENC {
        return "Error: input too large".to_string();
    }
    text.chars()
        .map(|c| match c {
            'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}

/// `/html/encode?text=<b>` -> "&lt;b&gt;"
#[get("/html/encode?<text>")]
pub fn html_encode(text: String) -> String {
    if text.len() > MAX_ENC {
        return "Error: input too large".to_string();
    }
    text.chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// `/html/decode?text=&lt;b&gt;` -> "<b>"
#[get("/html/decode?<text>")]
pub fn html_decode(text: String) -> String {
    if text.len() > MAX_ENC {
        return "Error: input too large".to_string();
    }
    // Decode named entities first, then &amp; LAST to avoid double-unescaping.
    let s = text
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'");
    s.replace("&amp;", "&")
}

/// `/base64url/encode?text=hello` -> URL-safe base64 without padding
#[get("/base64url/encode?<text>")]
pub fn base64url_encode(text: String) -> String {
    if text.len() > 1_000_000 {
        return "Error: input too large (max 1MB)".to_string();
    }
    general_purpose::URL_SAFE_NO_PAD.encode(text.as_bytes())
}

/// `/base64url/decode?input=...`
#[get("/base64url/decode?<input>")]
pub fn base64url_decode(input: String) -> String {
    if input.len() > 1_500_000 {
        return "Error: input too large".to_string();
    }
    match general_purpose::URL_SAFE_NO_PAD.decode(input.trim()) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => "Error: invalid base64url".to_string(),
    }
}
