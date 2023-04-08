#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChatContent {
    pub answer: String,
    pub question: String,
    #[serde(alias = "answerImage")]
    pub answer_image: Option<String>,
}
