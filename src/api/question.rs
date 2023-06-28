use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::Client;
impl Client {
    /// Get answers to a question
    /// 
    /// it takes the context and question as arguments
    pub async fn get_question(
        &self,
        context: String,
        question: String,
    ) -> Result<Answer, Box<dyn std::error::Error>> {
        log::trace!("getting answers");

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.config.key).parse()?,
        );
        log::trace!("authenticating with {}", self.config.key);

        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);

        let client = reqwest::Client::new();
        let body = serde_json::to_string(&QuestionQuery { question, context })?;
        log::info!("sending request for model {}", self.config.question_model);
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
        let answer: Result<Answer, serde_json::Error> = serde_json::from_str(&res);
        if let Err(e) = answer {
            log::error!("error: {}", e);
            return Err(Box::new(e));
        }
        Ok(answer?)
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
