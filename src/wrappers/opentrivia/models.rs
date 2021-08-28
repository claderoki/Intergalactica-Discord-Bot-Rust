#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Multiple,
    Boolean,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(serde::Deserialize, Debug)]
pub struct Trivia {
    pub category: String,
    #[serde(rename = "type")]
    pub kind: Kind,
    pub difficulty: Difficulty,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}
