pub use public::*;

pub mod public {
    use crate::chatgpt_request::private;
    use crate::settings::ChatGptSettings;

    pub async fn chatgpt_request(settings: &ChatGptSettings) {
        // TODO how to wait async function to finish?
        println!("chatgpt_request");
        private::request(settings).await.and_then(|_| {
            println!("chatgpt_request THEN");
            Ok(())
        });
    }
}

mod private {
    use reqwest::Error;
    use serde::Deserialize;

    use crate::settings::ChatGptSettings;

    #[derive(Deserialize)]
    struct ApiResponse {
        choices: Vec<Choice>,
    }

    #[derive(Deserialize)]
    struct Choice {
        text: String,
    }

    pub async fn request_models(settings: &ChatGptSettings) -> Result<(), Error> {
        let client = reqwest::Client::new();
        println!("make model request");
        let res = client
            .get("https://api.openai.com/v1/models")
            .header("Content-Type", "application/json")
            .header("Authorization", ["Bearer ", &settings.api_key].join(" "))
            .send()
            .await;

        // TODO proper error handling
        match res {
            Ok(res) => {
                println!("model request finished {:?}", res.status());
                let body = res.text().await?; // TODO json wants struct?
                println!("Body: {}", serde_json::to_string_pretty(&body).unwrap());
            }
            Err(e) => println!("model request error {:?}", e),
        }

        // TODO handle error properly

        Ok(())
    }

    pub async fn request(settings: &ChatGptSettings) -> Result<(), Error> {
        // TODO just to test if we can make a succesfull API request
        request_models(settings).await.and_then(|_| {
            println!("model request THEN");
            Ok(())
        });
        println!("Model request done");

        let client = reqwest::Client::new();
        println!("make request");
        let res = client
            .post("https://api.openai.com/v1/engines/davinci/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", ["Bearer ", &settings.api_key].join(" "))
            .body(r#"{"prompt": "Hello, how are you?", "max_tokens": 150}"#)
            .send()
            .await;

        match res {
            Ok(res) => {
                println!("prompt request finished {:?}", res.status());
                let body = res.text().await?; // TODO json wants struct?
                println!("Body: {}", serde_json::to_string_pretty(&body).unwrap());
            }
            Err(e) => println!("prompt request error {:?}", e),
        }

        // TODO handle error
        // println!("request finished {:?}", res.status());
        // let api_response: ApiResponse = res.json().await?;
        // println!(
        //     "Response: {:?}",
        //     api_response.choices.first().map(|choice| &choice.text)
        // );

        Ok(())
    }
}
