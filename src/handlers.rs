use crate::chat::{message_chatgpt, new_conversation};
use chatgpt::converse::Conversation;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct SummarizeHandler {}

impl SummarizeHandler {
    pub fn new() -> Self {
        SummarizeHandler {}
    }

    pub async fn handle(&self, message: &String) -> Result<String, Error> {
        let response = Self::summarize(message).await?;

        Ok(response)
    }

    pub async fn summarize(message: &String) -> Result<String, Error> {
        const PROMPT: &str = "Please summarize the following text in as much detail as possible. \
            \n\nText: \n";

        let query = format!("{PROMPT}{message}");
        let response = message_chatgpt(&query).await?;

        Ok(response)
    }
}

pub struct ChatHandler {
    pub conversations: Option<HashMap<serenity::UserId, Conversation>>,
}

impl ChatHandler {
    pub fn new() -> Self {
        ChatHandler {
            conversations: Some(HashMap::new()),
        }
    }

    pub async fn handle(
        &mut self,
        message: &String,
        author_id: serenity::UserId,
    ) -> Result<String, Error> {
        let conversation = self._get_conversation(author_id)?;
        let response = conversation.send_message(message).await?;
        let response = response.message().content.clone();

        Ok(response)
    }

    pub fn _get_conversation(
        &mut self,
        author_id: serenity::UserId,
    ) -> Result<&mut Conversation, Error> {
        let conversations = self.conversations.as_mut().ok_or("Couldn't get context")?;
        let conversation = conversations
            .entry(author_id)
            .or_insert_with(new_conversation);

        Ok(conversation)
    }
}
