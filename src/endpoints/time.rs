use std::path::PathBuf;

use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use rocket::serde::json::{serde_json, Json, Value};

use crate::types::{TimeResponse, TimestampResponse};

// Time endpoints
#[get("/timestamp")]
pub fn timestamp() -> Json<TimestampResponse> {
    let now = Utc::now();
    Json(TimestampResponse {
        seconds: now.timestamp(),
        milliseconds: now.timestamp_millis(),
    })
}

/// Convert a unix timestamp to a human-readable date.
/// `/timestamp/1781022640` -> formatted UTC + ISO8601. Auto-detects ms vs s.
#[get("/timestamp/<unix>")]
pub fn timestamp_to_date(unix: String) -> Json<Value> {
    let unix: i64 = match unix.trim().parse() {
        Ok(n) => n,
        Err(_) => return Json(serde_json::json!({"error": "timestamp must be an integer"})),
    };
    // Heuristic: values beyond ~year 5138 in seconds are almost certainly milliseconds.
    let (secs, unit) = if unix.abs() > 100_000_000_000 {
        (unix / 1000, "milliseconds")
    } else {
        (unix, "seconds")
    };
    match Utc.timestamp_opt(secs, 0).single() {
        Some(dt) => Json(serde_json::json!({
            "unix": unix,
            "interpreted_as": unit,
            "utc": dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            "iso8601": dt.to_rfc3339(),
        })),
        None => Json(serde_json::json!({"error": "invalid timestamp"})),
    }
}

#[get("/time/utc")]
pub fn time_utc() -> Json<TimeResponse> {
    let now = Utc::now();
    Json(TimeResponse {
        datetime: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        timezone: "UTC".to_string(),
    })
}

/// Current time in an IANA timezone, e.g. `/time/America/New_York` or `/time/Asia/Dubai`.
/// Uses `<tz..>` so multi-segment zone names like `America/New_York` work.
#[get("/time/<tz..>")]
pub fn time_tz(tz: PathBuf) -> Json<TimeResponse> {
    let name = tz.to_string_lossy().to_string();
    match name.parse::<Tz>() {
        Ok(zone) => {
            let now = Utc::now().with_timezone(&zone);
            Json(TimeResponse {
                datetime: now.format("%Y-%m-%d %H:%M:%S").to_string(),
                timezone: zone.name().to_string(),
            })
        }
        Err(_) => Json(TimeResponse {
            datetime: String::new(),
            timezone: format!(
                "unknown timezone '{}' (use IANA names like America/New_York or Asia/Dubai)",
                name
            ),
        }),
    }
}

/// Humanize a duration in seconds. `/duration?seconds=90061` -> "1d 1h 1m 1s"
#[get("/duration?<seconds>")]
pub fn duration(seconds: Option<String>) -> Json<Value> {
    let seconds: i64 = match seconds.as_deref().map(str::trim).map(str::parse) {
        Some(Ok(n)) => n,
        _ => return Json(serde_json::json!({"error": "provide ?seconds=<integer>"})),
    };
    let s = seconds.abs();
    let days = s / 86_400;
    let hours = (s % 86_400) / 3_600;
    let mins = (s % 3_600) / 60;
    let secs = s % 60;

    let mut parts: Vec<String> = Vec::new();
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if mins > 0 {
        parts.push(format!("{}m", mins));
    }
    if secs > 0 || parts.is_empty() {
        parts.push(format!("{}s", secs));
    }

    Json(serde_json::json!({
        "seconds": seconds,
        "human": parts.join(" "),
        "days": days,
        "hours": hours,
        "minutes": mins,
    }))
}
