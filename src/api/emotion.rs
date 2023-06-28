use reqwest::header;
use serde::Deserialize;

use crate::Client;
impl Client {
    pub async fn get_emotions(
        &self,
        string: String,
    ) -> Result<Vec<Mood>, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.config.key).parse()?,
        );
        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);

        let client = reqwest::Client::new();
        let res = client
            .post(format!(
                "https://api-inference.huggingface.co/models/{}",
                self.config.emotion_model
            ))
            .headers(headers)
            .body(format!("inputs={}", string))
            .send()
            .await?
            .text()
            .await?;
        let mood: Result<Vec<Vec<Mood>>, serde_json::Error> = serde_json::from_str(&res);
        Ok(mood?.into_iter().flatten().collect())
    }
}
#[derive(Deserialize)]
pub struct Mood {
    pub label: String,
    pub score: f32,
}
#[cfg(test)]
mod tests {
    use crate::{Client, Config};

    #[tokio::test]
    async fn emotion() {
        let mut config = Config::default();
        config.key = std::env::var("HUGGINGFACE_API_KEY").expect("HUGGINGFACE_API_KEY not set");
        let client = Client::new(config);
        let emotions = client.get_emotions("hello there".to_string()).await;
        assert!(emotions.is_ok());
    }
}
