use crate::chat::new_conversation;
use chatgpt::converse::Conversation;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
type Error = Box<dyn std::error::Error + Send + Sync>;

pub enum CommandType {
    Chat,
    Summarize,
}

impl CommandType {
    pub fn from_str(command: &str) -> Result<Self, Error> {
        match command {
            "chat" => Ok(CommandType::Chat),
            "summarize" => Ok(CommandType::Summarize),
            _ => Err("Unknown command".into()),
        }
    }
}

pub struct CommandHandler {
    pub conversations: Option<HashMap<serenity::UserId, Conversation>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler {
            conversations: Some(HashMap::new()),
        }
    }

    pub async fn handle(
        &mut self,
        command: &str,
        message: &str,
        author_id: serenity::UserId,
    ) -> Result<String, Error> {
        let command = CommandType::from_str(command)?;

        let response = match command {
            CommandType::Chat => self.chat(message, author_id).await?,
            CommandType::Summarize => self.summarize(message, author_id).await?,
        };

        Ok(response)
    }

    pub async fn chat(
        &mut self,
        message: &str,
        author_id: serenity::UserId,
    ) -> Result<String, Error> {
        let conversation = self._get_conversation(author_id)?;
        let response = conversation.send_message(message).await?;
        let response = response.message().content.clone();

        Ok(response)
    }

    pub async fn summarize(
        &mut self,
        message: &str,
        author_id: serenity::UserId,
    ) -> Result<String, Error> {
        const SUMMARIZE_PROMPT: &str =
            "Please summarize the following text in as much detail as possible. \
            \n\nText: \n";
        let summarize_message = format!("{SUMMARIZE_PROMPT}{message}");

        let conversation = self._get_conversation(author_id)?;
        let response = conversation.send_message(&summarize_message).await?;
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
