pub use public::*;

pub mod public {
    use crate::chatgpt_request::private;
    use crate::debug_logger::DebugLogger;
    use crate::settings::ChatGptSettings;

    pub async fn chatgpt_request(
        my_prompt: &String,
        settings: &ChatGptSettings,
        log: &Box<dyn DebugLogger>,
    ) -> Result<String, String> {
        match private::request(my_prompt, settings, &log).await {
            Ok(response_text) => {
                log.debug(&"Chat request finished");
                Ok(response_text)
            }
            Err(e) => Err(e),
        }
    }
}

mod private {
    use reqwest::Error;
    use serde::{Deserialize, Serialize};

    use crate::debug_logger::{debug_as_json, DebugLogger};
    use crate::settings::ChatGptSettings;

    /// For ChatGPT API doc see https://platform.openai.com/docs/api-reference/

    static API_ROOT: &str = "https://api.openai.com/v1";
    static DEFAULT_TEMPERATURE: f32 = 0.8;

    // "model": "gpt-3.5-turbo-16k",
    // "messages": [{"role": "user", "content": "How to use ChatGPT API with cURL?"}]
    #[derive(Serialize)]
    struct PromptRequest {
        model: String,
        messages: Vec<PromptRequestMessage>,
        temperature: f32, // TODO valid range 0-2 ?
                          // TODO there also other props in API doc
    }

    #[derive(Serialize)]
    struct PromptRequestMessage {
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
        message: PromptResponseMessage,
        // logprobs: PromptResponseLogProbs,
        finish_reason: String, // like "length" or "stop", "functional_call" TODO how to map to enum
    }

    #[derive(Deserialize, Serialize)]
    struct PromptResponseMessage {
        role: String,
        content: String,
    }

    pub async fn request(
        my_prompt: &String,
        settings: &ChatGptSettings,
        log: &Box<dyn DebugLogger>,
    ) -> Result<String, String> {
        // TODO calculate how much time takes to make request
        // TODO wrap requests parameters to own class that can be serialized, perhaps builder pattern

        let your_struct = PromptRequest {
            model: settings.model.clone(),
            messages: vec![PromptRequestMessage {
                role: "user".to_string(),
                content: my_prompt.clone(),
            }],
            temperature: DEFAULT_TEMPERATURE,
        };

        // TODO would optional message be better, or just add extra msg to debug as json?
        log.debug(&"Request body:");
        debug_as_json(log, &your_struct);

        let client = reqwest::Client::new();
        log.debug(&"Start request...");
        let res = client
            .post(format!("{API_ROOT}/completions"))
            .header("Content-Type", "application/json")
            .header("Authorization", ["Bearer ", &settings.api_key].join(" "))
            .json(&your_struct)
            .send()
            .await;

        let response = match res {
            Ok(res) => {
                log.debug(&format!(
                    "Prompt request finished: status_code={:?}",
                    res.status()
                ));

                let body_json = match res.text().await {
                    Ok(response_text) => {
                        log.debug_d(&response_text);

                        let prompt_response: PromptResponse =
                            match serde_json::from_slice(&response_text.as_bytes()) {
                                Ok(my_struct) => my_struct,
                                Err(e) => {
                                    let error_msg = format!(
                                    "ERROR: Failed to parse response to PromptResponse json: {:?}",
                                    e
                                );
                                    log.debug(&error_msg);
                                    Err(error_msg)?
                                }
                            };

                        prompt_response
                    }

                    Err(e) => {
                        let error_msg =
                            format!("ERROR: Failed get response body in bytes: {:?}", e);
                        log.debug(&error_msg);
                        Err(error_msg)?
                    }
                };

                let response = match body_json
                    .choices
                    .first()
                    .map(|choice| &choice.message.content)
                {
                    Some(text) => text,
                    None => "No response",
                };

                response.to_string()
            }
            Err(e) => {
                // todo why this additional message approach gives compiler error?
                let error_msg = format!("ERROR: Failed get response: {:?}", e);
                log.debug(&error_msg);
                Err(error_msg)?
            }
        };

        // TODO now passing always Ok, but should pass error if request fails
        Ok(response)
    }

    pub async fn request_models(
        settings: &ChatGptSettings,
        log: &Box<dyn DebugLogger>,
    ) -> Result<(), Error> {
        let client = reqwest::Client::new();
        log.debug(&"Start model request...");
        let res = client
            .get(format!("{API_ROOT}/models"))
            .header("Content-Type", "application/json")
            .header("Authorization", ["Bearer ", &settings.api_key].join(" "))
            .send()
            .await;

        // TODO proper error handling
        match res {
            Ok(res) => {
                log.debug(&format!(
                    "model request finished: status_code={:?}",
                    res.status()
                ));
                let body = res.text().await?; // TODO json wants struct?
                log.debug(&format!(
                    "Body: {}",
                    serde_json::to_string_pretty(&body).unwrap()
                ));
            }
            Err(e) => println!("ERROR: model request error {:?}", e),
        }

        // TODO handle error properly

        Ok(())
    }
}
