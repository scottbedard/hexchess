use hexchess::San;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SearchResult {
    /// depth of search
    pub depth: u8,

    /// number of times the evaluation function was executed
    pub evaluations: u32,

    /// ordered list of possible sans, sorted by score best to worst
    pub sans: Vec<ScoredSan>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ScoredSan {
    /// fen of the position
    #[serde(serialize_with = "serialize_san_as_string")]
    pub san: San,

    /// score of the position
    pub score: f32,
}

fn serialize_san_as_string<S>(san: &San, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&san.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_san_from_and_to_string() {
        assert_eq!(1, 1);
    }
}
