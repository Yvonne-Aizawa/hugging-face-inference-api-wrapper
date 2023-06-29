use std::error::Error;

use reqwest::header;
use serde::Deserialize;

use crate::Client;
impl Client {
    /// Get token classifications from a string
    pub async fn get_classifications(
        &self,
        string: String,
    ) -> Result<Vec<Classification>, Box<dyn Error + Send + Sync>> {
        log::trace!("getting classifications");
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.config.key).parse()?,
        );
        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);
        log::trace!("authenticating with {}", self.config.key);
        let client = reqwest::Client::new();
        log::info!(
            "sending request for model {}",
            self.config.classification_model
        );
        let res = client
            .post(format!(
                "https://api-inference.huggingface.co/models/{}",
                self.config.classification_model
            ))
            .headers(headers)
            .body(format!("inputs={}", string))
            .send()
            .await?
            .text()
            .await?;

        let classifications: Result<Vec<Classification>, serde_json::Error> =
            serde_json::from_str(&res);
        if let Err(e) = classifications {
            log::error!("error: {}", e);
            return Err(Box::new(e));
        }
        Ok(classifications?)
    }
}
#[derive(Deserialize, Debug)]
pub struct Classification {
    pub entity_group: String,
    pub score: f32,
    pub word: String,
    pub start: usize,
    pub end: usize,
}
#[cfg(test)]
mod tests {
    use crate::{Client, Config};

    #[tokio::test]
    async fn classification() {
        let mut config = Config::default();
        config.key = std::env::var("HUGGINGFACE_API_KEY").expect("HUGGINGFACE_API_KEY not set");
        let client = Client::new(config);
        let classification = client
            .get_classifications("hello i am Yvonne Take i live in Amsterdam".to_string())
            .await;
        assert!(classification.is_ok());
    }
}
