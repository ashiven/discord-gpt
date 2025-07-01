use crate::chat::new_conversation;
use chatgpt::converse::Conversation;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
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
        duration: Option<u64>,
        author_id: serenity::UserId,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Session {
    pub duration: Duration,
    pub start_time: SystemTime,
}

pub struct CommandHandler {
    pub conversations: HashMap<serenity::UserId, Conversation>,
    pub sessions: HashMap<serenity::UserId, Session>,
}

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler {
            conversations: HashMap::new(),
            sessions: HashMap::new(),
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
        duration: Option<u64>,
        author_id: serenity::UserId,
    ) -> Result<String, Error> {
        let session = self._get_session(duration, author_id)?;
        let session_duration = session.duration.as_secs() / 60;
        let session_runtime = session.start_time.elapsed()?;
        let runtime_minutes = session_runtime.as_secs() / 60;
        let runtime_seconds = session_runtime.as_secs() % 60;

        if session_runtime < session.duration {
            let response = format!(
                "Your {session_duration} minute session has been running for: \n\n {runtime_minutes}m : {runtime_seconds}s"
            );
            return Ok(response);
        }

        self.sessions.remove(&author_id);
        let end_session_prompt = format!("I have just completed a pomodoro session that lasted \
            for {runtime_minutes} minutes. Please inform me in a creative way that my session has ended \
            and that I can take a break now.\n");
        let conversation = self._get_conversation(author_id)?;
        let response = conversation.send_message(&end_session_prompt).await?;
        let response = response.message().content.clone();

        Ok(response)
    }

    /// If the user provided a duration, we want to create
    /// a new pomodoro session with that duration and return it.
    /// If the user did not provide a duration, we want to
    /// return the existing session or a default session of 60 minutes.
    pub fn _get_session(
        &mut self,
        duration: Option<u64>,
        author_id: serenity::UserId,
    ) -> Result<Session, Error> {
        if let Some(duration) = duration {
            self.sessions.insert(
                author_id,
                Session {
                    duration: Duration::from_secs(duration * 60),
                    start_time: SystemTime::now(),
                },
            );
        }
        let session = self.sessions.entry(author_id).or_insert(Session {
            duration: Duration::from_secs(60 * 60),
            start_time: SystemTime::now(),
        });

        Ok(*session)
    }

    pub fn _get_conversation(
        &mut self,
        author_id: serenity::UserId,
    ) -> Result<&mut Conversation, Error> {
        let conversation = self
            .conversations
            .entry(author_id)
            .or_insert_with(new_conversation);

        Ok(conversation)
    }
}
