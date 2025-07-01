use crate::chat::new_conversation;
use chatgpt::converse::Conversation;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
type Error = Box<dyn std::error::Error + Send + Sync>;

pub enum Command<'a> {
    Chat {
        message: &'a str,
        author_id: serenity::UserId,
    },
    Summarize {
        message: &'a str,
        author_id: serenity::UserId,
    },
    Session {
        duration: u64,
        author_id: serenity::UserId,
    },
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

    pub async fn handle(&mut self, command: Command<'_>) -> Result<String, Error> {
        let response = match command {
            Command::Chat { message, author_id } => self.chat(message, author_id).await?,
            Command::Summarize { message, author_id } => self.summarize(message, author_id).await?,
            Command::Session {
                duration,
                author_id,
            } => self.session(duration, author_id).await?,
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

    pub async fn session(
        &mut self,
        duration: u64,
        author_id: serenity::UserId,
    ) -> Result<String, Error> {
        let prompt = format!(
            "You are starting a new session that will last for {duration} minutes. \
            \n\nPlease confirm the start of the session."
        );
        let conversation = self._get_conversation(author_id)?;
        let response = conversation.send_message(&prompt).await?;
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
