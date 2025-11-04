mod utils;

use wasm_bindgen::prelude::*;
use hexchess::Hexchess;

#[wasm_bindgen]
extern "C" {
    // ...
}

#[wasm_bindgen]
pub fn init() -> String {
    Hexchess::init().to_string()
}
