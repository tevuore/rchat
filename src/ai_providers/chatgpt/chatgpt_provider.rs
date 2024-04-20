use crate::ai_providers::chatgpt::chatgpt_request::request;
use crate::debug_logger::DebugLogger;
use crate::settings::ChatGptSettings;

// TODO impl struct implementing AIProvider trait
pub async fn ai_request(
    my_prompt: &String,
    settings: &ChatGptSettings,
    log: &Box<dyn DebugLogger>,
) -> Result<String, String> {
    match request(my_prompt, settings, &log).await {
        Ok(response_text) => {
            log.debug(&"Chat request finished");
            Ok(response_text)
        }
        Err(e) => Err(e),
    }
}
