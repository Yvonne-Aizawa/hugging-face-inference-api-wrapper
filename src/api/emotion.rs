use std::error::Error;

use reqwest::header;
use serde::Deserialize;

use crate::Client;

impl Client {
    /// Get emotions from a string of text
    pub async fn get_emotions(
        &self,
        string: String,
    ) -> Result<Vec<Mood>, Box<dyn Error + Send + Sync>> {
        log::trace!("getting emotions");

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.config.key).parse()?,
        );
        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);

        let client = reqwest::Client::new();
        log::info!("sending request for model {}", self.config.emotion_model);

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
        if let Err(e) = mood {
            log::error!("error: {}", e);
            return Err(Box::new(e));
        }
        Ok(mood?.into_iter().flatten().collect())
    }
}
#[derive(Deserialize , Debug)]
pub struct Mood {
    pub label: MoodLabel,
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
//the inputs are not capitalized so we need to do that
#[derive(Deserialize, Debug)]
pub enum MoodLabel {
    #[serde(rename = "disappointment")]
    Disappointment,
    #[serde(rename = "sadness")]
    Sadness,
    #[serde(rename = "annoyance")]
    Annoyance,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "disapproval")]
    Disapproval,
    #[serde(rename = "realization")]
    Realization,
    #[serde(rename = "nervousness")]
    Nervousness,
    #[serde(rename = "approval")]
    Approval,
    #[serde(rename = "joy")]
    Joy,
    #[serde(rename = "anger")]
    Anger,
    #[serde(rename = "embarrassment")]
    Embarrassment,
    #[serde(rename = "caring")]
    Caring,
    #[serde(rename = "remorse")]
    Remorse,
    #[serde(rename = "disgust")]
    Disgust,
    #[serde(rename = "grief")]
    Grief,
    #[serde(rename = "confusion")]
    Confusion,
    #[serde(rename = "relief")]
    Relief,
    #[serde(rename = "desire")]
    Desire,
    #[serde(rename = "admiration")]
    Admiration,
    #[serde(rename = "optimism")]
    Optimism,
    #[serde(rename = "fear")]
    Fear,
    #[serde(rename = "love")]
    Love,
    #[serde(rename = "excitement")]
    Excitement,
    #[serde(rename = "curiosity")]
    Curiosity,
    #[serde(rename = "amusement")]
    Amusement,
    #[serde(rename = "surprise")]
    Surprise,
    #[serde(rename = "gratitude")]
    Gratitude,
    #[serde(rename = "pride")]
    Pride,
}