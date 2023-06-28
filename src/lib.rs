mod api;
pub struct Config {
    pub key: String,
    pub emotion_model: String,
    pub question_model: String,
    pub classification_model: String,
}
impl Config {
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
    pub fn default() -> Self {
        let config = Config {
            key: String::from(""),
            emotion_model: String::from("SamLowe/roberta-base-go_emotions"),
            question_model: String::from("deepset/tinyroberta-squad2"),
            classification_model: String::from("dbmdz/bert-large-cased-finetuned-conll03-english"),
        };
        config
    }
}

pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config }
    }
}
