use rocket::serde::json::{serde_json, Json, Value};

// SECURITY: cap text inputs so no single request can exhaust CPU/memory.
const MAX_TEXT: usize = 10_000;

fn too_large_str() -> String {
    "Error: input too large (max 10000 chars)".to_string()
}

/// `/text/slugify?text=Hello World!` -> `hello-world`
#[get("/text/slugify?<text>")]
pub fn slugify(text: String) -> String {
    if text.len() > MAX_TEXT {
        return too_large_str();
    }
    let mut slug = String::new();
    let mut prev_dash = false;
    for c in text.to_lowercase().chars() {
        if c.is_alphanumeric() {
            slug.push(c);
            prev_dash = false;
        } else if !prev_dash && !slug.is_empty() {
            slug.push('-');
            prev_dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    slug
}

/// `/text/reverse?text=abc` -> `cba`
#[get("/text/reverse?<text>")]
pub fn reverse(text: String) -> String {
    if text.len() > MAX_TEXT {
        return too_large_str();
    }
    text.chars().rev().collect()
}

/// `/text/count?text=...` -> counts of chars/bytes/words/lines
#[get("/text/count?<text>")]
pub fn count(text: String) -> Json<Value> {
    if text.len() > MAX_TEXT {
        return Json(serde_json::json!({"error": "input too large (max 10000 chars)"}));
    }
    Json(serde_json::json!({
        "characters": text.chars().count(),
        "bytes": text.len(),
        "words": text.split_whitespace().count(),
        "lines": if text.is_empty() { 0 } else { text.lines().count() },
    }))
}

/// `/text/case/<mode>?text=...` where mode = upper|lower|title|snake|kebab|camel
#[get("/text/case/<mode>?<text>")]
pub fn case(mode: String, text: String) -> String {
    if text.len() > MAX_TEXT {
        return too_large_str();
    }
    match mode.to_lowercase().as_str() {
        "upper" => text.to_uppercase(),
        "lower" => text.to_lowercase(),
        "title" => text
            .split_whitespace()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
        "snake" => text
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("_"),
        "kebab" => text
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-"),
        "camel" => {
            let mut out = String::new();
            for (i, w) in text.split_whitespace().enumerate() {
                let mut ch = w.chars();
                if let Some(f) = ch.next() {
                    if i == 0 {
                        out.push_str(&f.to_lowercase().collect::<String>());
                    } else {
                        out.push_str(&f.to_uppercase().collect::<String>());
                    }
                    out.push_str(&ch.as_str().to_lowercase());
                }
            }
            out
        }
        _ => "Error: unsupported case (upper, lower, title, snake, kebab, camel)".to_string(),
    }
}
