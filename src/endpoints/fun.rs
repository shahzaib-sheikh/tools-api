use rand::Rng;
use rocket::serde::json::{serde_json, Json, Value};

use crate::constants::{CAT_FACTS, EIGHT_BALL, QUOTES};

// Fun endpoints
#[get("/cat-fact")]
pub fn cat_fact() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..CAT_FACTS.len());
    CAT_FACTS[index].to_string()
}

#[get("/quote")]
pub fn quote() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..QUOTES.len());
    QUOTES[index].to_string()
}

/// Roll dice in `NdM` notation, e.g. `/roll/2d6`. Capped at 100 dice / 1000 sides.
#[get("/roll/<dice>")]
pub fn roll(dice: String) -> Json<Value> {
    let lower = dice.to_lowercase();
    let parts: Vec<&str> = lower.split('d').collect();
    if parts.len() != 2 {
        return Json(serde_json::json!({"error": "format: NdM, e.g. 2d6"}));
    }

    let count: u32 = if parts[0].is_empty() {
        1
    } else {
        match parts[0].parse() {
            Ok(v) => v,
            Err(_) => return Json(serde_json::json!({"error": "invalid dice count"})),
        }
    };
    let sides: u32 = match parts[1].parse() {
        Ok(v) => v,
        Err(_) => return Json(serde_json::json!({"error": "invalid number of sides"})),
    };

    if !(1..=100).contains(&count) {
        return Json(serde_json::json!({"error": "dice count must be 1-100"}));
    }
    if !(2..=1000).contains(&sides) {
        return Json(serde_json::json!({"error": "sides must be 2-1000"}));
    }

    let mut rng = rand::thread_rng();
    let rolls: Vec<u32> = (0..count).map(|_| rng.gen_range(1..=sides)).collect();
    let total: u64 = rolls.iter().map(|&r| r as u64).sum();

    Json(serde_json::json!({
        "dice": dice,
        "rolls": rolls,
        "total": total,
    }))
}

#[get("/coinflip")]
pub fn coinflip() -> Json<Value> {
    let mut rng = rand::thread_rng();
    Json(serde_json::json!({
        "result": if rng.gen::<bool>() { "heads" } else { "tails" }
    }))
}

#[get("/8ball")]
pub fn eight_ball() -> Json<Value> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..EIGHT_BALL.len());
    Json(serde_json::json!({"answer": EIGHT_BALL[index]}))
}

/// Pick a random option. `/pick?options=red,green,blue`
#[get("/pick?<options>")]
pub fn pick(options: String) -> Json<Value> {
    if options.len() > 10_000 {
        return Json(serde_json::json!({"error": "input too large"}));
    }
    let opts: Vec<&str> = options
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if opts.is_empty() {
        return Json(serde_json::json!({"error": "provide comma-separated options, e.g. a,b,c"}));
    }
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..opts.len());
    Json(serde_json::json!({
        "options": opts,
        "chosen": opts[index],
    }))
}
