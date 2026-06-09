use rocket::serde::json::{serde_json, Json, Value};

/// Convert an integer between bases 2..=36.
/// `/base/convert?value=255&from=10&to=16` -> "ff"
#[get("/base/convert?<value>&<from>&<to>")]
pub fn base_convert(value: String, from: Option<String>, to: Option<String>) -> Json<Value> {
    if value.len() > 256 {
        return Json(serde_json::json!({"error": "input too large"}));
    }
    let (from, to) = match (
        from.as_deref().map(str::trim).map(str::parse::<u32>),
        to.as_deref().map(str::trim).map(str::parse::<u32>),
    ) {
        (Some(Ok(f)), Some(Ok(t))) => (f, t),
        _ => {
            return Json(serde_json::json!({
                "error": "provide from and to as integer bases, e.g. ?value=255&from=10&to=16"
            }))
        }
    };
    if !(2..=36).contains(&from) || !(2..=36).contains(&to) {
        return Json(serde_json::json!({"error": "base must be between 2 and 36"}));
    }
    match i128::from_str_radix(value.trim(), from) {
        Ok(n) => Json(serde_json::json!({
            "input": value,
            "from_base": from,
            "to_base": to,
            "decimal": n.to_string(),
            "result": to_radix(n, to),
        })),
        Err(_) => Json(serde_json::json!({
            "error": format!("'{}' is not a valid base-{} number", value, from)
        })),
    }
}

fn to_radix(n: i128, radix: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let neg = n < 0;
    let mut x = n.unsigned_abs();
    let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let r = radix as u128;
    let mut out = Vec::new();
    while x > 0 {
        out.push(digits[(x % r) as usize]);
        x /= r;
    }
    if neg {
        out.push(b'-');
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

/// `/color/hex-to-rgb?hex=ff8800`
#[get("/color/hex-to-rgb?<hex>")]
pub fn hex_to_rgb(hex: String) -> Json<Value> {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 || !h.chars().all(|c| c.is_ascii_hexdigit()) {
        return Json(serde_json::json!({"error": "expected a 6-digit hex color like ff8800"}));
    }
    let r = u8::from_str_radix(&h[0..2], 16).unwrap();
    let g = u8::from_str_radix(&h[2..4], 16).unwrap();
    let b = u8::from_str_radix(&h[4..6], 16).unwrap();
    Json(serde_json::json!({
        "hex": format!("#{}", h.to_lowercase()),
        "r": r, "g": g, "b": b,
        "rgb": format!("rgb({}, {}, {})", r, g, b),
    }))
}

/// `/color/rgb-to-hex?r=255&g=136&b=0`
#[get("/color/rgb-to-hex?<r>&<g>&<b>")]
pub fn rgb_to_hex(r: Option<String>, g: Option<String>, b: Option<String>) -> Json<Value> {
    let parse = |o: Option<String>| -> Option<i64> {
        o.as_deref().and_then(|s| s.trim().parse::<i64>().ok())
    };
    let (r, g, b) = match (parse(r), parse(g), parse(b)) {
        (Some(r), Some(g), Some(b)) => (r, g, b),
        _ => {
            return Json(serde_json::json!({
                "error": "provide r, g, b as integers, e.g. ?r=255&g=136&b=0"
            }))
        }
    };
    // Range-check explicitly so the message always fires (not swallowed by integer parsing).
    if !(0..=255).contains(&r) || !(0..=255).contains(&g) || !(0..=255).contains(&b) {
        return Json(serde_json::json!({"error": "each channel must be 0-255"}));
    }
    Json(serde_json::json!({
        "hex": format!("#{:02x}{:02x}{:02x}", r as u8, g as u8, b as u8),
        "r": r, "g": g, "b": b,
    }))
}
