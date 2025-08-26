use std::cmp;
use std::time::Duration;
use rocket::response::status;
use rocket::tokio::time::sleep;

// Utility endpoints
#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> &'static str {
    // Limit delay to prevent DoS attacks
    let max_delay = cmp::min(seconds, 30); // Max 30 seconds
    sleep(Duration::from_secs(max_delay)).await;
    "OK"
}

#[get("/status/<code>")]
pub fn status_code(code: u16) -> status::Custom<&'static str> {
    status::Custom(rocket::http::Status::from_code(code).unwrap_or(rocket::http::Status::Ok), "")
}
