#[macro_use]
extern crate rocket;

mod constants;
mod endpoints;
mod extractors;
mod types;

use rocket_dyn_templates::Template;

use endpoints::{basic, convert, crypto, encoding, fun, generators, qr, text, time, utils};

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
            encoding::hex_encode,
            encoding::hex_decode,
            encoding::rot13,
            encoding::html_encode,
            encoding::html_decode,
            encoding::base64url_encode,
            encoding::base64url_decode,
            crypto::hash,
            crypto::jwt_decode,
            time::timestamp,
            time::timestamp_to_date,
            time::time_utc,
            time::time_tz,
            time::duration,
            text::slugify,
            text::reverse,
            text::count,
            text::case,
            convert::base_convert,
            convert::hex_to_rgb,
            convert::rgb_to_hex,
            fun::cat_fact,
            fun::quote,
            fun::roll,
            fun::coinflip,
            fun::eight_ball,
            fun::pick,
            qr::qr,
        ])
}

#[launch]
fn rocket() -> _ {
    create_rocket()
}

#[cfg(test)]
mod route_tests {
    use super::create_rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use rocket::serde::json::serde_json::Value;

    fn client() -> Client {
        Client::tracked(create_rocket()).expect("rocket failed to ignite")
    }

    fn get_text(path: &str) -> (Status, String) {
        let c = client();
        let res = c.get(path).dispatch();
        let status = res.status();
        (status, res.into_string().unwrap_or_default())
    }

    fn get_json(path: &str) -> (Status, Value) {
        let c = client();
        let res = c.get(path).dispatch();
        let status = res.status();
        let body = res.into_string().unwrap_or_default();
        let value: Value = rocket::serde::json::serde_json::from_str(&body)
            .unwrap_or_else(|_| panic!("non-JSON body for {}: {}", path, body));
        (status, value)
    }

    // ---- sanity / existing ----
    #[test]
    fn ping_works() {
        let (s, b) = get_text("/ping");
        assert_eq!(s, Status::Ok);
        assert_eq!(b, "pong");
    }

    #[test]
    fn index_renders() {
        let (s, _) = get_text("/");
        assert_eq!(s, Status::Ok);
    }

    #[test]
    fn uuid_has_correct_shape() {
        let (s, b) = get_text("/uuid");
        assert_eq!(s, Status::Ok);
        assert_eq!(b.len(), 36);
        assert_eq!(b.matches('-').count(), 4);
    }

    // ---- time ----
    #[test]
    fn timestamp_to_date_seconds() {
        let (s, v) = get_json("/timestamp/1781022640");
        assert_eq!(s, Status::Ok);
        assert_eq!(v["interpreted_as"], "seconds");
        assert_eq!(v["utc"], "2026-06-09 16:30:40 UTC");
    }

    #[test]
    fn timestamp_to_date_detects_millis() {
        let (_, v) = get_json("/timestamp/1781022640255");
        assert_eq!(v["interpreted_as"], "milliseconds");
    }

    #[test]
    fn time_tz_resolves_iana_zone() {
        let (s, v) = get_json("/time/Asia/Dubai");
        assert_eq!(s, Status::Ok);
        assert_eq!(v["timezone"], "Asia/Dubai");
        assert!(!v["datetime"].as_str().unwrap().is_empty());
    }

    #[test]
    fn time_tz_rejects_bad_zone() {
        let (_, v) = get_json("/time/Not/AZone");
        assert!(v["timezone"].as_str().unwrap().contains("unknown timezone"));
    }

    #[test]
    fn duration_humanizes() {
        let (s, v) = get_json("/duration?seconds=90061");
        assert_eq!(s, Status::Ok);
        assert_eq!(v["human"], "1d 1h 1m 1s");
        assert_eq!(v["days"], 1);
    }

    // ---- text ----
    #[test]
    fn slugify_basic() {
        assert_eq!(get_text("/text/slugify?text=Hello%20World%21").1, "hello-world");
    }

    #[test]
    fn reverse_basic() {
        assert_eq!(get_text("/text/reverse?text=stressed").1, "desserts");
    }

    #[test]
    fn count_basic() {
        let (_, v) = get_json("/text/count?text=the%20quick%20brown%20fox");
        assert_eq!(v["words"], 4);
        assert_eq!(v["characters"], 19);
    }

    #[test]
    fn case_modes() {
        assert_eq!(get_text("/text/case/upper?text=hi%20yo").1, "HI YO");
        assert_eq!(get_text("/text/case/title?text=hello%20world").1, "Hello World");
        assert_eq!(get_text("/text/case/snake?text=hello%20world").1, "hello_world");
        assert_eq!(get_text("/text/case/kebab?text=hello%20world").1, "hello-world");
        assert_eq!(get_text("/text/case/camel?text=hello%20world").1, "helloWorld");
    }

    // ---- encoding ----
    #[test]
    fn hex_roundtrip() {
        assert_eq!(get_text("/hex/encode?text=hi").1, "6869");
        assert_eq!(get_text("/hex/decode?input=6869").1, "hi");
    }

    #[test]
    fn rot13_is_self_inverse() {
        assert_eq!(get_text("/rot13?text=Hello").1, "Uryyb");
        assert_eq!(get_text("/rot13?text=Uryyb").1, "Hello");
    }

    #[test]
    fn html_encode_decode_roundtrip() {
        assert_eq!(get_text("/html/encode?text=%3Cb%3Ehi%3C%2Fb%3E").1, "&lt;b&gt;hi&lt;/b&gt;");
        assert_eq!(get_text("/html/decode?text=%26lt%3Bb%26gt%3Bhi%26lt%3B%2Fb%26gt%3B").1, "<b>hi</b>");
    }

    #[test]
    fn base64url_encodes_without_padding() {
        let b = get_text("/base64url/encode?text=hello%20world").1;
        assert_eq!(b, "aGVsbG8gd29ybGQ");
        assert!(!b.contains('='));
    }

    // ---- convert ----
    #[test]
    fn base_convert_dec_to_hex() {
        let (_, v) = get_json("/base/convert?value=255&from=10&to=16");
        assert_eq!(v["result"], "ff");
    }

    #[test]
    fn base_convert_rejects_bad_base() {
        let (_, v) = get_json("/base/convert?value=10&from=10&to=99");
        assert!(v["error"].is_string());
    }

    #[test]
    fn color_conversions_roundtrip() {
        let (_, v) = get_json("/color/hex-to-rgb?hex=ff8800");
        assert_eq!(v["r"], 255);
        assert_eq!(v["g"], 136);
        let (_, v2) = get_json("/color/rgb-to-hex?r=255&g=136&b=0");
        assert_eq!(v2["hex"], "#ff8800");
    }

    #[test]
    fn rgb_rejects_out_of_range() {
        let (_, v) = get_json("/color/rgb-to-hex?r=999&g=0&b=0");
        assert!(v["error"].is_string());
    }

    // ---- fun ----
    #[test]
    fn roll_within_bounds() {
        let (s, v) = get_json("/roll/3d6");
        assert_eq!(s, Status::Ok);
        let rolls = v["rolls"].as_array().unwrap();
        assert_eq!(rolls.len(), 3);
        for r in rolls {
            let n = r.as_u64().unwrap();
            assert!((1..=6).contains(&n));
        }
    }

    #[test]
    fn roll_rejects_excessive_dice() {
        let (_, v) = get_json("/roll/9999d6");
        assert!(v["error"].is_string());
    }

    #[test]
    fn coinflip_is_heads_or_tails() {
        let (_, v) = get_json("/coinflip");
        let r = v["result"].as_str().unwrap();
        assert!(r == "heads" || r == "tails");
    }

    #[test]
    fn pick_chooses_from_options() {
        let (_, v) = get_json("/pick?options=red,green,blue");
        let chosen = v["chosen"].as_str().unwrap();
        assert!(["red", "green", "blue"].contains(&chosen));
    }

    // ---- qr / limits ----
    #[test]
    fn qr_returns_svg() {
        let c = client();
        let res = c.get("/qr?text=hello").dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert_eq!(res.content_type(), Some(ContentType::SVG));
        assert!(res.into_string().unwrap().contains("<svg"));
    }

    #[test]
    fn qr_rejects_oversized_input() {
        let big = "a".repeat(2000);
        let (_, b) = get_text(&format!("/qr?text={}", big));
        assert!(b.starts_with("Error:"));
    }

    #[test]
    fn text_endpoints_enforce_size_cap() {
        let big = "a".repeat(20_000);
        let (_, b) = get_text(&format!("/text/reverse?text={}", big));
        assert!(b.starts_with("Error:"));
    }
}
