use rand::Rng;
use crate::constants::{CAT_FACTS, QUOTES};

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
