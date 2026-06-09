use qrcode::render::svg;
use qrcode::QrCode;
use rocket::http::ContentType;

// SECURITY: hard cap on input so QR generation can't be used to burn CPU/memory.
const MAX_QR_TEXT: usize = 1_000;

/// `/qr?text=hello` -> an SVG QR code (Content-Type: image/svg+xml).
#[get("/qr?<text>")]
pub fn qr(text: String) -> (ContentType, String) {
    if text.is_empty() {
        return (ContentType::Plain, "Error: provide ?text=...".to_string());
    }
    if text.len() > MAX_QR_TEXT {
        return (
            ContentType::Plain,
            format!("Error: text too large (max {} chars)", MAX_QR_TEXT),
        );
    }
    match QrCode::new(text.as_bytes()) {
        Ok(code) => {
            let image = code
                .render::<svg::Color>()
                .min_dimensions(200, 200)
                .max_dimensions(600, 600)
                .build();
            (ContentType::SVG, image)
        }
        Err(_) => (
            ContentType::Plain,
            "Error: input exceeds QR code capacity".to_string(),
        ),
    }
}
