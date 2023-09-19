pub use public::*;

pub mod public {
    use crate::chatgpt_request::private;
    use crate::debug_logger::DebugLogger;
    use crate::settings::ChatGptSettings;

    pub async fn chatgpt_request(
        my_prompt: &String,
        settings: &ChatGptSettings,
        log: &Box<dyn DebugLogger>) {
        private::request(my_prompt, settings, &log).await.and_then(|_| {
            log.debug(&"Chat request finished");
            Ok(())
        });
    }
}

mod private {
    use reqwest::Error;
    use serde::{Deserialize, Serialize};
    use crate::debug_logger::DebugLogger;
    use crate::settings::ChatGptSettings;

    #[derive(Deserialize, Serialize)]
    struct Message {
        role: String,
        content: String,
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
        message: Message,
        // logprobs: PromptResponseLogProbs,
        finish_reason: String, // like "length" or "stop"
    }

    // "model": "gpt-3.5-turbo-16k",
    // "messages": [{"role": "user", "content": "How to use ChatGPT API with cURL?"}]
    #[derive(Serialize)]
    struct PromptRequest {
        model: String,
        messages: Vec<PromptRequestMessage>,
    }

    #[derive(Serialize)]
    struct PromptRequestMessage {
        role: String,
        content: String,
    }

    // TODO instead of printing directly into stdout, return the response and use a printer abstraction

    pub async fn request(
        my_prompt: &String,
        settings: &ChatGptSettings,
        log: &Box<dyn DebugLogger>) -> Result<(), Error> {

        // TODO calculate how much time takes to make request
        // TODO wrap requests parameters to own class that can be serialized, perhaps builder pattern

        println!("ME: {}", my_prompt);
        let your_struct = PromptRequest {
            model: settings.model.clone(),
            messages: vec![PromptRequestMessage {
                role: "user".to_string(),
                content: my_prompt.clone(),
            }],
        };

        let serialized_json = serde_json::to_string(&your_struct).unwrap();
        // TODO now serialization happens even if debug is not enabled
        log.debug(&format!("request: {}", serialized_json));

        // TODO debug output ongoing json
        let client = reqwest::Client::new();
        log.debug(&"Start request...");
        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", ["Bearer ", &settings.api_key].join(" "))
            .json(&your_struct)
            .send()
            .await;

        match res {
            Ok(res) => {
                log.debug(&format!("Prompt request finished: status_code={:?}", res.status()));

                let body = match res.json::<PromptResponse>().await {
                    Ok(res) => {
                        // TODO full json only if debug
                        log.debug(&format!("Body: {}", serde_json::to_string_pretty(&res).unwrap()));
                        res
                    }
                    Err(e) => {
                        println!("ERROR: Prompt request error {:?}", e);
                        Err(e)?
                    }
                };

                let response = match body.choices.first().map(|choice| &choice.message.content) {
                    Some(text) => text,
                    None => "No response",
                };

                // TODO I get quotes around the response, why?
                println!("CHATGPT: {:?}", response);
            }
            Err(e) => {
                println!("ERROR: Prompt request error {:?}", e);
            }
        }

        // TODO now passing always Ok, but should pass error if request fails
        Ok(())
    }

    pub async fn request_models(
        settings: &ChatGptSettings,
        log: &Box<dyn DebugLogger>) -> Result<(), Error> {
        let client = reqwest::Client::new();
        log.debug(&"Start model request...");
        let res = client
            .get("https://api.openai.com/v1/models")
            .header("Content-Type", "application/json")
            .header("Authorization", ["Bearer ", &settings.api_key].join(" "))
            .send()
            .await;

        // TODO proper error handling
        match res {
            Ok(res) => {
                log.debug(&format!("model request finished: status_code={:?}", res.status()));
                let body = res.text().await?; // TODO json wants struct?
                log.debug(&format!("Body: {}", serde_json::to_string_pretty(&body).unwrap()));
            }
            Err(e) => println!("ERROR: model request error {:?}", e),
        }

        // TODO handle error properly

        Ok(())
    }
}
