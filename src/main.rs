#[macro_use]
extern crate rocket;

mod constants;
mod endpoints;
mod extractors;
mod types;

use rocket_dyn_templates::Template;

use endpoints::{basic, crypto, encoding, fun, generators, time, utils};

pub fn create_rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![
            basic::index,
            basic::whoami,
            basic::ip,
            basic::ip_info,
            basic::headers,
            basic::user_agent,
            basic::echo,
            basic::ping,
            utils::delay,
            utils::status_code,
            generators::generate_uuid,
            generators::lorem,
            generators::random_color,
            generators::generate_password,
            generators::random_number,
            encoding::base64_encode,
            encoding::base64_decode,
            encoding::url_encode,
            encoding::url_decode,
            crypto::hash,
            time::timestamp,
            time::time_utc,
            time::time_tz,
            crypto::jwt_decode,
            fun::cat_fact,
            fun::quote
        ])
}

#[launch]
fn rocket() -> _ {
    create_rocket()
}