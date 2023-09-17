pub use public::*;

pub mod public {
    use crate::chatgpt_request::private;
    use crate::settings::ChatGptSettings;

    pub async fn chatgpt_request(my_prompt: &String, settings: &ChatGptSettings) {
        // TODO how to wait async function to finish?
        //println!("chatgpt_request");
        private::request(my_prompt, settings).await.and_then(|_| {
            println!("chatgpt_request THEN");
            Ok(())
        });
    }
}

mod private {
    use reqwest::Error;
    use serde::{Deserialize, Serialize};

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

    // This `derive` requires the `serde` dependency.
    #[derive(Deserialize, Serialize)]
    struct PromptResponse {
        id: String,
        object: String,
        created: u64,
        model: String,
        usage: PromptResponseUsage,
        choices: Vec<PromptResponseChoice>,
    }

    #[derive(Deserialize, Serialize)]
    struct PromptResponseUsage {
        prompt_tokens: u32,
        completion_tokens: u32,
        total_tokens: u32,
    }

    #[derive(Deserialize, Serialize)]
    struct PromptResponseChoice {
        index: u32,
        text: String,
        // logprobs: PromptResponseLogProbs,
        finish_reason: String, // like "length"
    }

    #[derive(Serialize)]
    struct PromptRequest {
        prompt: String,
    }

    // TODO instead of printing directly into stdout, return the response and use a printer abstraction

    pub async fn request(my_prompt: &String, settings: &ChatGptSettings) -> Result<(), Error> {
        // TODO just to test if we can make a successful API request
        // request_models(settings).await.and_then(|_| {
        //     println!("model request THEN");
        //     Ok(())
        // });
        // println!("Model request done");

        // TODO calculate how much time takes to make request
        // TODO wrap requests parameters to own class that can be serialized, perhaps builder pattern

        println!("ME: {}", my_prompt);
        let your_struct = PromptRequest {
            prompt: my_prompt.to_string(),
        };

        let client = reqwest::Client::new();
        println!("make request");
        let res = client
            .post("https://api.openai.com/v1/engines/davinci/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", ["Bearer ", &settings.api_key].join(" "))
            .json(&your_struct) // TeroV add here my_prompt, json escape
            .send()
            .await;

        match res {
            Ok(res) => {
                println!("prompt request finished {:?}", res.status());
                //let body = res.json::<PromptResponse>().await?; // TODO json wants struct?

                let body = match res.json::<PromptResponse>().await {
                    Ok(res) => {
                        // TODO full json only if debug
                        println!("Body: {}", serde_json::to_string_pretty(&res).unwrap());
                        res
                    }
                    Err(e) => {
                        println!("prompt request error {:?}", e);
                        Err(e)?
                    }
                };

                let response = match body.choices.first().map(|choice| &choice.text) {
                    Some(text) => text,
                    None => "No response",
                };

                println!("CHATGPT: {:?}", response);
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
