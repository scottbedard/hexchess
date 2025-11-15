use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct EvaluateOptions {
    pub depth: u8,
    pub position: String,
}
