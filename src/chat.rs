use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;
use chatgpt::Result as ChatResult;

pub async fn message_chatgpt(message: &str) -> ChatResult<String> {
    let api_key = std::env::var("OPENAI_API_KEY")?;

    let client = ChatGPT::new(api_key)?;

    let response: CompletionResponse = client.send_message(message).await?;

    let message_content = response.message().content.clone();

    Ok(message_content)
}

pub fn new_conversation() -> Conversation {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let client = ChatGPT::new(api_key).unwrap();

    client.new_conversation()
}
