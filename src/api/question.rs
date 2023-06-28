use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::Client;
impl Client {
    pub async fn get_question(
        &self,
        context: String,
        question: String,
    ) -> Result<Answer, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.config.key).parse()?,
        );
        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);

        let client = reqwest::Client::new();
        // {
        // "inputs": {
        // "question": "What's my name?",
        // "context": "My name is Clara and I live in Berkeley."
        // }
        let body = serde_json::to_string(&QuestionQuery { question, context })?;

        let res = client
            .post(format!(
                "https://api-inference.huggingface.co/models/{}",
                self.config.question_model
            ))
            .headers(headers)
            .body(body)
            .send()
            .await?
            .text()
            .await?;
        dbg!(&res);
        let classifications: Result<Answer, serde_json::Error> = serde_json::from_str(&res);
        Ok(classifications?)
    }
}
#[derive(Serialize)]
struct QuestionQuery {
    question: String,
    context: String,
}
#[derive(Deserialize, Debug)]
pub struct Answer {
    pub score: f32,
    pub start: usize,
    pub end: usize,
    pub answer: String,
}
#[cfg(test)]
mod tests {
    use crate::{Client, Config};

    #[tokio::test]
    async fn question() {
        let mut config = Config::default();
        config.key = std::env::var("HUGGINGFACE_API_KEY").expect("HUGGINGFACE_API_KEY not set");
        let client = Client::new(config);
        let question = client
            .get_question(
                "hello i am Yvonne Take i live in Amsterdam".to_string(),
                "WHat is her name?".to_string(),
            )
            .await;
        dbg!(&question);
        assert!(question.is_ok());
    }
}
