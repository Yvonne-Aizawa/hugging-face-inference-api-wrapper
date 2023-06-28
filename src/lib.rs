
mod api;

/// Configuration for the client
pub struct Config {
    /// key: API key for the HuggingFace API
    pub key: String,
    /// emotion_model hugging face model defaults to SamLowe/roberta-base-go_emotions
    pub emotion_model: String,
    /// question_model hugging face model defaults to deepset/tinyroberta-squad2
    pub question_model: String,
    /// classification_model hugging face model defaults to dbmdz/bert-large-cased-finetuned-conll03-english
    pub classification_model: String,
}
impl Config {
    /// Create a new configuration
    pub fn new(
        key: String,
        emotion_model: String,
        question_model: String,
        classification_model: String,
    ) -> Self {
        Config {
            key,
            emotion_model,
            question_model,
            classification_model,
        }
    }
    /// Create a default configuration
    pub fn default() -> Self {
        log::trace!("generated default config");
        let config = Config {
            key: String::from(""),
            emotion_model: String::from("SamLowe/roberta-base-go_emotions"),
            question_model: String::from("deepset/tinyroberta-squad2"),
            classification_model: String::from("dbmdz/bert-large-cased-finetuned-conll03-english"),
        };
        config
    }
}

/// Client for the HuggingFace API
///
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config }
    }
}
