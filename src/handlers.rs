use std::collections::HashMap;

use crate::chat::{message_chatgpt, new_conversation};
use chatgpt::converse::Conversation;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use url::Url;

pub struct SummarizeHandler {}

impl SummarizeHandler {
    pub fn new() -> Self {
        SummarizeHandler {}
    }

    pub async fn handle(&self, msg: &Message) -> CommandResult<String> {
        // extract the message content exluding the first line (the command)
        let mut content = msg.content.clone();
        content = content.lines().skip(1).collect::<Vec<_>>().join("\n");

        let response;
        match &msg.referenced_message {
            Some(replied_to) => {
                response = Self::reply(replied_to).await?;
            }
            None => {
                let first_line = content.lines().find(|line| !line.is_empty()).ok_or("")?;
                if Url::parse(first_line).is_ok() {
                    response = Self::link(first_line).await?;
                } else {
                    response = Self::message(&content).await?;
                }
            }
        }
        Ok(response)
    }

    pub async fn message(content: &str) -> CommandResult<String> {
        const PROMPT: &str = "Please summarize the following text in as much detail as possible. \
            \n\nText: \n";

        let query = format!("{}{}", PROMPT, content);
        let chatgpt_response = message_chatgpt(&query).await?;

        Ok(chatgpt_response)
    }

    pub async fn reply(replied_to: &Message) -> CommandResult<String> {
        const PROMPT: &str = "The following is a message sent in a Discord channel. \
        Please summarize it in as much detail as possible. \
        \n\nText: \n";

        let content = replied_to.content.clone();
        let query = format!("{}{}", PROMPT, content);
        let chatgpt_response = message_chatgpt(&query).await?;

        Ok(chatgpt_response)
    }

    pub async fn link(url: &str) -> CommandResult<String> {
        const PROMPT: &str = "The following are the contents of a website. \
        Please summarize them in as much detail as possible. \
        \n\nContent: \n";

        let content = reqwest::get(url).await?.text().await?;
        // TODO: - do some stuff to get rid of html tags and only get the text
        let content = content.chars().take(4096).collect::<String>();
        let query = format!("{}{}", PROMPT, content);
        let chatgpt_response = message_chatgpt(&query).await?;

        Ok(chatgpt_response)
    }
}

pub struct ChatHandler {
    // a hashmap to store the conversation context for each user (keyed by user id)
    pub context: Option<HashMap<u64, Conversation>>,
}

impl ChatHandler {
    pub fn new() -> Self {
        ChatHandler {
            context: Some(HashMap::new()),
        }
    }

    pub async fn handle(&mut self, msg: &Message) -> CommandResult<String> {
        let mut content = msg.content.clone();
        content = content.lines().skip(1).collect::<Vec<_>>().join("\n");
        let user_id = msg.author.id.0;
        let context = self.context.as_mut().ok_or("Couldn't get context")?;

        let conversation = match context.get_mut(&user_id) {
            Some(conversation) => {
                conversation
            }
            None => {
                let conversation = new_conversation();
                context.insert(user_id, conversation);
                context
                    .get_mut(&user_id)
                    .ok_or("Couldn't get conversation")?
            }
        };

        let response = conversation.send_message(&content).await?;
        let response = response.message().content.clone();

        Ok(response)
    }
}
