use bytes::Bytes;
use reqwest::{Error, Response, StatusCode};
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

    let req = build_request(
        my_prompt,
        settings,
    );

    let res = make_request(
        req,
        settings,
        log,
    ).await;

    let prompt_response = process_response_result(res, log).await?;

    Ok(prompt_response)
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

fn build_request(
    my_prompt: &String,
    settings: &ChatGptSettings,
) -> PromptRequest {
    PromptRequest {
        model: settings.model.clone(),
        messages: vec![PromptRequestMessage {
            role: "user".to_string(),
            content: my_prompt.clone(),
        }],
        temperature: DEFAULT_TEMPERATURE,
    }
}

async fn make_request(
    your_struct: PromptRequest,
    settings: &ChatGptSettings,
    log: &Box<dyn DebugLogger>,
) -> Result<Response, Error> {

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

    res
}

// TODO how getting partial responses matches to impl? Now awaits all data
async fn process_response_result(
    response: Result<Response, Error>,
    log: &Box<dyn DebugLogger>,
) -> Result<String, String> {
    match response {
        Ok(res) => process_response_msg(res, log).await,
        Err(e) => {
            // todo why this additional message approach gives compiler error?
            let error_msg = format!("ERROR: Failed get response: {:?}", e);
            log.debug(&error_msg);
            Err(error_msg)?
        }
    }
}

async fn process_response_msg(
    res: Response,
    log: &Box<dyn DebugLogger>,
) -> Result<String, String> {

    let http_status = res.status();
    let response_bytes = get_bytes(res, log).await?;

    if http_status != StatusCode::OK {
        process_error_msg(&http_status, &response_bytes, log)?;
    }

    let prompt_response = parse_response_msg(&response_bytes, log)?;

    // TODO we should have some kind of generic struct for response
    let response = match prompt_response
        .choices
        .first()
        .map(|choice| &choice.message.content)
    {
        Some(text) => text,
        None => "No response",
    };

    Ok(response.to_string())
}

async fn get_bytes(
    res: Response,
    log: &Box<dyn DebugLogger>,
) -> Result<Bytes, String> {
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

    Ok(response_bytes)
}

// TODO now error is always String, should be error type?
fn process_error_msg(
    http_status: &StatusCode,
    response_bytes: &Bytes,
    log: &Box<dyn DebugLogger>,
) -> Result<String, String> {
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
                return Err(error_msg);
            }
        };
    let err_str = format!(
        "Response error {}: {}",
        error_response.error.code, error_response.error.message
    );
    return Err(err_str); // TODO more specific error types? benefit of thiserror?}
}

fn parse_response_msg(
    response_bytes: &Bytes,
    log: &Box<dyn DebugLogger>,
) -> Result<PromptResponse, String> {
    match serde_json::from_slice(&response_bytes)
    {
        Ok(my_struct) => Ok(my_struct),
        Err(e) => {
            let error_msg = format!(
                "ERROR: Failed to parse response to PromptResponse json: {:?}",
                e
            );
            log.debug(&error_msg);
            Err(error_msg)?
        }
    }
}