use serde::{Deserialize, Serialize};

// "model": "gpt-3.5-turbo-16k",
// "messages": [{"role": "user", "content": "How to use ChatGPT API with cURL?"}]
#[derive(Serialize)]
pub struct PromptRequest {
    pub model: String,
    pub messages: Vec<PromptRequestMessage>,
    pub temperature: f32, // TODO valid range 0-2 ?
    // TODO there also other props in API doc
}

#[derive(Serialize)]
pub struct PromptRequestMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct PromptResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub usage: PromptResponseUsage,
    pub choices: Vec<PromptResponseChoice>,
}

#[derive(Deserialize, Serialize)]
pub struct PromptResponseUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Serialize)]
pub struct PromptResponseChoice {
    pub index: u32,
    pub message: PromptResponseMessage,
    // logprobs: PromptResponseLogProbs,
    pub finish_reason: String, // like "length" or "stop", "functional_call" TODO how to map to enum
}

#[derive(Deserialize, Serialize)]
pub struct PromptResponseMessage {
    pub role: String,
    pub content: String,
}

// {
//     "error": {
//         "message": "You exceeded your current quota, please check your plan and billing details. For more information on this error, read the docs: https://platform.openai.com/docs/guides/error-codes/api-errors.",
//         "type": "insufficient_quota",
//         "param": null,
//         "code": "insufficient_quota"
//     }
// }
#[derive(Deserialize)]
pub struct PromptResponseErrorMessage {
    pub error: PromptResponseError,
}

#[derive(Deserialize)]
pub struct PromptResponseError {
    pub message: String,
    pub r#type: String,
    pub param: Option<String>,
    pub code: String,
}
