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

    pub async fn handle(&self, msg: serenity::Message) -> Result<String, Error> {
        let mut content = msg.content.clone();
        content = content.lines().skip(1).collect::<Vec<_>>().join("\n");
        let response = match msg.referenced_message {
            Some(replied_to) => Self::reply(*replied_to).await?,
            None => Self::message(&content).await?,
        };
        Ok(response)
    }

    pub async fn message(content: &str) -> Result<String, Error> {
        const PROMPT: &str = "Please summarize the following text in as much detail as possible. \
            \n\nText: \n";

        let query = format!("{PROMPT}{content}");
        let chatgpt_response = message_chatgpt(&query).await?;

        Ok(chatgpt_response)
    }

    pub async fn reply(replied_to: serenity::Message) -> Result<String, Error> {
        const PROMPT: &str = "The following is a message sent in a Discord channel. \
        Please summarize it in as much detail as possible. \
        \n\nText: \n";

        let content = replied_to.content.clone();
        let query = format!("{PROMPT}{content}");
        let chatgpt_response = message_chatgpt(&query).await?;

        Ok(chatgpt_response)
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

    pub async fn handle(&mut self, msg: serenity::Message) -> Result<String, Error> {
        let mut content = msg.content.clone();
        content = content.lines().skip(1).collect::<Vec<_>>().join("\n");
        let user_id = msg.author.id;
        let conversations = self.conversations.as_mut().ok_or("Couldn't get context")?;

        let conversation = match conversations.get_mut(&user_id) {
            Some(conversation) => conversation,
            None => {
                let conversation = new_conversation();
                conversations.insert(user_id, conversation);
                conversations
                    .get_mut(&user_id)
                    .ok_or("Couldn't get conversation")?
            }
        };

        let response = conversation.send_message(&content).await?;
        let response = response.message().content.clone();

        Ok(response)
    }
}
