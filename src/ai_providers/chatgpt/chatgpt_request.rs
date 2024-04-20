use reqwest::Error;
use serde::{Deserialize, Serialize};
use crate::ai_providers::chatgpt::chatgpt_messages::{PromptRequest, PromptRequestMessage, PromptResponse, PromptResponseErrorMessage};

use crate::debug_logger::{debug_as_json, DebugLogger};
use crate::settings::ChatGptSettings;

/// For ChatGPT API doc see https://platform.openai.com/docs/api-reference/

static API_ROOT: &str = "https://api.openai.com/v1";
static DEFAULT_TEMPERATURE: f32 = 0.8;

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
    log.debug(&"...");
    let url = format!("{API_ROOT}/chat/completions");
    log.debug(&format!("Start request {url}"));

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", ["Bearer ", &settings.api_key].join(" "))
        .json(&your_struct)
        .send()
        .await;

    let response = match res {
        Ok(res) => {
            // as we have response, we should have something to read, ok or error message
            let http_status = res.status();
            let bytes_result = res.bytes().await;

            let response_bytes = match bytes_result {
                Ok(response_bytes) => response_bytes,
                Err(e) => {
                    let error_msg =
                        format!("ERROR: Failed read response body in bytes: {:?}", e);
                    log.debug(&error_msg);
                    Err(error_msg)?
                }
            };
            if log.enabled() {
                // TeroV following  will be already json formatted, so need to pretty print again => in general yes
                let debug_str = String::from_utf8_lossy(&*response_bytes);
                log.debug_d(&debug_str.to_string());
            }

            log.debug_d(&format!("Got response with bytes {}", response_bytes.len()));

            // there might be error although sending itself worked fine
            if http_status != reqwest::StatusCode::OK {
                let error_msg = format!(
                    "ERROR: Prompt request failed: status_code={:?}",
                    http_status
                );
                log.debug(&error_msg);
                let error_response: PromptResponseErrorMessage =
                    match serde_json::from_slice(&response_bytes) {
                        Ok(error_msg) => error_msg,
                        Err(e) => {
                            let error_msg = format!(
                                "ERROR: Failed to parse error response from json: {}",
                                e
                            );
                            log.debug(&error_msg);
                            Err(error_msg)?
                        }
                    };
                let err_str = format!(
                    "Response error {}: {}",
                    error_response.error.code, error_response.error.message
                );
                Err(err_str)? // TODO more specific error types? benefit of thiserror?
            }

            let prompt_response: PromptResponse = match serde_json::from_slice(&response_bytes)
            {
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

            let response = match prompt_response
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