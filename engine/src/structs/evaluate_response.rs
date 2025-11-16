use crate::structs::ScoredSan;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct EvaluateResponse {
    /// depth of search
    pub depth: u8,

    /// number of times the evaluation function was executed
    pub evaluations: u32,

    /// ordered list of possible sans, sorted by score best to worst
    pub sans: Vec<ScoredSan>,
}
