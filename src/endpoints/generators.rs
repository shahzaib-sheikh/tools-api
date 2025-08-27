use std::cmp;
use rand::Rng;
use uuid::Uuid;
use crate::constants::LOREM_WORDS;

#[get("/uuid")]
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

// Generator endpoints - PERFORMANCE: Using static array
#[get("/lorem/<words>")]
pub fn lorem(words: usize) -> String {
    // Limit words to prevent resource exhaustion
    let safe_words = cmp::min(words, 1000); // Max 1000 words
    if safe_words == 0 {
        return "Error: Word count must be greater than 0".to_string();
    }
    
    let mut rng = rand::thread_rng();
    let mut result = Vec::with_capacity(safe_words);
    
    for _ in 0..safe_words {
        let word_index = rng.gen_range(0..LOREM_WORDS.len());
        result.push(LOREM_WORDS[word_index]);
    }
    
    result.join(" ")
}

#[get("/color")]
pub fn random_color() -> String {
    let mut rng = rand::thread_rng();
    format!("#{:06x}", rng.gen::<u32>() & 0xFFFFFF)
}

#[get("/password/<length>")]
pub fn generate_password(length: usize) -> String {
    // Limit password length to prevent resource exhaustion
    let safe_length = cmp::min(length, 128); // Max 128 characters
    if safe_length == 0 {
        return "Error: Password length must be greater than 0".to_string();
    }
    
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
    let mut rng = rand::thread_rng();
    
    (0..safe_length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

#[get("/number/<min>/<max>")]
pub fn random_number(min: i32, max: i32) -> String {
    // SECURITY FIX: Validate input ranges to prevent panic
    if min > max {
        return "Error: min value cannot be greater than max value".to_string();
    }
    
    // Prevent extreme ranges that could cause performance issues
    let range_size = (max as i64) - (min as i64);
    if range_size > 1_000_000_000 {
        return "Error: range too large (max 1 billion)".to_string();
    }
    
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max).to_string()
}
