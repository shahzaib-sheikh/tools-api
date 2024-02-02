use std::net::IpAddr;

use rocket::figment::value::Map;
use rocket::request::FromRequest;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::Request;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WhoamiResponse {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let mut headers: Map<String, String> = Map::new();

        for h in request.headers().iter() {
            headers.insert(h.name().to_string(), h.value().to_string());
        }

        let mut cookie_map: Map<String, String> = Map::new();

        for c in request.cookies().iter() {
            cookie_map.insert(c.name().to_string(), c.value().to_string());
        }

        let remote = request.remote();
        let default_ip: IpAddr = "127.0.0.1".parse().unwrap();

        rocket::request::Outcome::Success(WhoamiResponse {
            ip: remote.map_or(default_ip, |rem| rem.ip()).to_string(),
            cookies: cookie_map,
            headers: headers,
        })
    }
}

#[derive(Serialize)]
struct WhoamiResponse {
    ip: String,
    cookies: Map<String, String>,
    headers: Map<String, String>,
}
#[get("/whoami")]
fn whoami(whoami: WhoamiResponse) -> Json<WhoamiResponse> {
    Json(whoami)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, whoami])
}
