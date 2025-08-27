use chrono::Utc;
use rocket::serde::json::Json;
use crate::types::{TimestampResponse, TimeResponse};

// Time endpoints
#[get("/timestamp")]
pub fn timestamp() -> Json<TimestampResponse> {
    let now = Utc::now();
    Json(TimestampResponse {
        seconds: now.timestamp(),
        milliseconds: now.timestamp_millis(),
    })
}

#[get("/time/utc")]
pub fn time_utc() -> Json<TimeResponse> {
    let now = Utc::now();
    Json(TimeResponse {
        datetime: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        timezone: "UTC".to_string(),
    })
}

#[get("/time/<tz>")]
pub fn time_tz(tz: String) -> Json<TimeResponse> {
    let now = Utc::now();
    
    // For simplicity, we'll just return UTC time with the requested timezone name
    // In a production app, you'd want to use a proper timezone library
    Json(TimeResponse {
        datetime: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        timezone: tz,
    })
}
