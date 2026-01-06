use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Core,
    Stretch,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiQuestion {
    pub prompt: String,
    pub answer: String,
    pub difficulty: Difficulty,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DayResponse {
    pub version: u32,
    pub grade: u8,
    pub day_id: usize,
    pub date_ymd: String, // YYYY-MM-DD
    pub items: Vec<AiQuestion>,
    pub source: String, // "ai" | "fallback"
}