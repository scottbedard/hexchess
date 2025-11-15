use hexchess::San;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Tsify, Serialize, Deserialize)]
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
