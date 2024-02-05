use std::net::IpAddr;

use rocket::figment::value::Map;
use rocket::request::FromRequest;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::Request;
use rocket_dyn_templates::{context, Template};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { field: "value" })
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WhoamiResponse {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let mut headers: Map<String, String> = Map::new();

        for h in request.headers().iter() {
            let header_name = h.name().to_string();
            if !header_name.starts_with("x-") {
                headers.insert(header_name, h.value().to_string());
            }
        }

        let mut cookie_map: Map<String, String> = Map::new();

        for c in request.cookies().iter() {
            cookie_map.insert(c.name().to_string(), c.value().to_string());
        }

        let default_ip: IpAddr = "127.0.0.1".parse().unwrap();

        let remote_ip: IpAddr = headers
            .get("x-real-ip")
            .map_or(default_ip, |ip| ip.parse().unwrap());

        rocket::request::Outcome::Success(WhoamiResponse {
            ip: remote_ip.to_string(),
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
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, whoami])
}
