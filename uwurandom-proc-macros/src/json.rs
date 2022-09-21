use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarkovArr {
    pub choices: Vec<Choice>,
    pub total_probability: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub next_ngram: usize,
    pub next_char: char,
    pub cumulative_probability: u32,
}
