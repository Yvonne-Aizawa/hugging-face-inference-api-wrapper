# Hugging face api wrapper

## what is this?
I use the hugging face inference api. i wrote a wrapper for this. currently it can detect emotions in text, detect places,people in text and answer a question about a text

## example use

```toml
[dependencies]
huggingface_inference_rs = "0.3.0"
tokio = { version =  "1.28.2", features = ["rt-multi-thread", "macros"] }
```
```rust
#[tokio::main]
async fn main() {
    let mut config = hg_api::Config::default();
    config.key = "hf_key".to_string();
    let client = hg_api::Client::new(config);
    let test_string = "This is the story of a man named Stanley. Stanley worked for a company in a big building where he was Employee #427. Employee #427's job was simple: he sat at his desk in Room 427 and he pushed buttons on a keyboard. ".to_string();
    let emotions = client.get_emotions(test_string.to_owned()).await;
    let classifications = client.get_classifications(test_string.to_owned()).await;
    let answer = client
        .get_question(
            test_string,
            "what employee number does stanly have?".to_string(),
        )
        .await;

    match emotions {
        Ok(emotions) => {
            for emotion in emotions {
                println!("{},{}", emotion.label, emotion.score);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    match classifications {
        Ok(classifications) => {
            for classification in classifications {
                println!("{},{}", classification.entity_group, classification.word);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    match answer {
        Ok(answer) => {
            println!("{}", answer.answer)
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

```






