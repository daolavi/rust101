mod utils;

use chrono::{DateTime, Local};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let now: DateTime<Local> = Local::now();
    alert(&format!("{:?}", now));
}
