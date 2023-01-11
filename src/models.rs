use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Word {
    pub id: u32,
    pub word: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WordDetail {
    pub id: u32,
    pub word: String,
    pub meaning: String,
    pub usage: Option<String>,
    pub date: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWord {
    pub word: String,
    pub meaning: String,
    pub usage: Option<String>,
}
