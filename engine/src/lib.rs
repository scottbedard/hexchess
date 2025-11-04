use crate::structs::SearchResult;
use hexchess::Hexchess;
use wasm_bindgen::prelude::*;

mod evaluation;
mod negamax;
mod ordering;
mod structs;

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn evaluate(fen: String, depth: u8) -> SearchResult {
    set_panic_hook();

    let hexchess = Hexchess::parse(&fen).unwrap();

    negamax::search(&hexchess, depth)
}
