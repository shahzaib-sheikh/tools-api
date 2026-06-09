use qrcode::render::svg;
use qrcode::QrCode;
use rocket::http::ContentType;

use barcoders::generators::svg::SVG as BarcodeSVG;
use barcoders::sym::code39::Code39;
use barcoders::sym::code128::Code128;
use barcoders::sym::ean13::EAN13;

use datamatrix::{DataMatrix, SymbolList};

// SECURITY: hard caps on input so code generation can't be used to burn CPU/memory.
const MAX_QR_TEXT: usize = 1_000;
const MAX_BARCODE_TEXT: usize = 200;
const MAX_DM_TEXT: usize = 500;

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

/// `/barcode?text=ABC123&symbology=code128` -> SVG 1D barcode.
/// symbology: code128 (default), code39, ean13.
#[get("/barcode?<text>&<symbology>")]
pub fn barcode(text: String, symbology: Option<String>) -> (ContentType, String) {
    if text.is_empty() {
        return (ContentType::Plain, "Error: provide ?text=...".to_string());
    }
    if text.len() > MAX_BARCODE_TEXT {
        return (
            ContentType::Plain,
            format!("Error: text too large (max {} chars)", MAX_BARCODE_TEXT),
        );
    }
    let sym = symbology.unwrap_or_else(|| "code128".to_string()).to_lowercase();

    let encoded: Result<Vec<u8>, String> = match sym.as_str() {
        "code39" => Code39::new(&text)
            .map(|b| b.encode())
            .map_err(|_| "invalid data for Code39 (use A-Z, 0-9, space, -.$/+%)".to_string()),
        "ean13" => EAN13::new(&text)
            .map(|b| b.encode())
            .map_err(|_| "invalid data for EAN-13 (use 12-13 digits)".to_string()),
        "code128" => {
            // Code128 in barcoders needs a leading code-set char; Ɓ = code set B (ASCII).
            Code128::new(format!("\u{0181}{}", text))
                .map(|b| b.encode())
                .map_err(|_| "invalid data for Code128".to_string())
        }
        _ => return (ContentType::Plain, "Error: unknown symbology (code128, code39, ean13)".to_string()),
    };

    match encoded {
        Ok(enc) => match BarcodeSVG::new(120).generate(&enc[..]) {
            Ok(s) => {
                // barcoders omits the SVG xmlns, which breaks <img>/standalone rendering.
                let s = if s.contains("xmlns") {
                    s
                } else {
                    s.replacen("<svg", "<svg xmlns=\"http://www.w3.org/2000/svg\"", 1)
                };
                (ContentType::SVG, s)
            }
            Err(_) => (ContentType::Plain, "Error: failed to render barcode".to_string()),
        },
        Err(msg) => (ContentType::Plain, format!("Error: {}", msg)),
    }
}

/// `/datamatrix?text=hello` -> SVG Data Matrix (ECC 200).
#[get("/datamatrix?<text>")]
pub fn data_matrix(text: String) -> (ContentType, String) {
    if text.is_empty() {
        return (ContentType::Plain, "Error: provide ?text=...".to_string());
    }
    if text.len() > MAX_DM_TEXT {
        return (
            ContentType::Plain,
            format!("Error: text too large (max {} chars)", MAX_DM_TEXT),
        );
    }
    match DataMatrix::encode(text.as_bytes(), SymbolList::default()) {
        Ok(code) => (ContentType::SVG, datamatrix_to_svg(&code)),
        Err(_) => (ContentType::Plain, "Error: could not encode Data Matrix".to_string()),
    }
}

fn datamatrix_to_svg(code: &datamatrix::DataMatrix) -> String {
    let bitmap = code.bitmap();
    let w = bitmap.width();
    let h = bitmap.height();
    let scale = 8usize;
    let quiet = 1usize; // 1-module quiet zone
    let total_w = (w + 2 * quiet) * scale;
    let total_h = (h + 2 * quiet) * scale;
    let mut rects = String::new();
    for (x, y) in bitmap.pixels() {
        let px = (x + quiet) * scale;
        let py = (y + quiet) * scale;
        rects.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
            px, py, scale, scale
        ));
    }
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" shape-rendering=\"crispEdges\"><rect width=\"100%\" height=\"100%\" fill=\"#fff\"/><g fill=\"#000\">{}</g></svg>",
        total_w, total_h, rects
    )
}
