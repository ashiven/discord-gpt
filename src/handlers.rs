use crate::chat::new_conversation;
use crate::{Context, Error};
use chatgpt::converse::Conversation;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

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
        ctx: Context<'a>,
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

    pub async fn handle(&mut self, command: Command<'_>) -> Result<Option<String>, Error> {
        let response = match command {
            Command::Chat { message, author_id } => self.chat(message, author_id).await?,
            Command::Summarize { message, author_id } => self.summarize(message, author_id).await?,
            Command::Session {
                ctx,
                duration,
                author_id,
            } => self.session(ctx, duration, author_id).await?,
        };

        Ok(response)
    }

    pub async fn chat(
        &mut self,
        message: &str,
        author_id: serenity::UserId,
    ) -> Result<Option<String>, Error> {
        let conversation = self._get_conversation(author_id)?;
        let response = conversation.send_message(message).await?;
        let response = response.message().content.clone();

        Ok(Some(response))
    }

    pub async fn summarize(
        &mut self,
        message: &str,
        author_id: serenity::UserId,
    ) -> Result<Option<String>, Error> {
        const SUMMARIZE_PROMPT: &str =
            "Please summarize the following text in as much detail as possible. \
            \n\nText: \n";
        let summarize_message = format!("{SUMMARIZE_PROMPT}{message}");

        let conversation = self._get_conversation(author_id)?;
        let response = conversation.send_message(&summarize_message).await?;
        let response = response.message().content.clone();

        Ok(Some(response))
    }

    async fn _end_session(
        &mut self,
        session_duration: u64,
        author_id: serenity::UserId,
    ) -> Result<String, Error> {
        self.sessions.remove(&author_id);
        let end_session_prompt = format!("I have just completed a pomodoro session that lasted \
            for {session_duration} minutes. Please inform me in a creative way that my session has ended \
            and that I can take a break now. You may use emojis in your response.\n");
        let conversation = self._get_conversation(author_id)?;
        let response = conversation.send_message(&end_session_prompt).await?;
        let response = response.message().content.clone();

        Ok(response)
    }

    pub async fn session(
        &mut self,
        ctx: Context<'_>,
        duration: Option<u64>,
        author_id: serenity::UserId,
    ) -> Result<Option<String>, Error> {
        let session = self._create_session(duration, author_id);
        if session.is_none() {
            self._reply(ctx, "You already have an active session".into())
                .await?;
            return Ok(None);
        }
        let session = session.unwrap();
        let session_duration = session.duration.as_secs() / 60;
        let start_text = format!("Starting your {session_duration} minute pomodoro session...\n");
        let reply_handle = self._reply(ctx, start_text).await?;

        while session.start_time.elapsed()? < session.duration {
            let session_runtime = session.start_time.elapsed()?;
            let runtime_minutes = session_runtime.as_secs() / 60;
            let runtime_seconds = session_runtime.as_secs() % 60;
            let runtime_text = format!(
                "Your {session_duration} minute session has been running for: \n\n {runtime_minutes}m : {runtime_seconds}s"
            );
            self._edit_reply(ctx, &reply_handle, runtime_text).await?;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        self._edit_reply(ctx, &reply_handle, "~Session Completed~".into())
            .await?;

        let end_text = self._end_session(session_duration, author_id).await?;
        self._reply(ctx, end_text).await?;
        Ok(None)
    }

    async fn _reply<'a>(
        &self,
        ctx: Context<'a>,
        content: String,
    ) -> Result<poise::ReplyHandle<'a>, Error> {
        let reply_handle = ctx.reply(content).await?;

        Ok(reply_handle)
    }

    async fn _edit_reply<'a>(
        &self,
        ctx: Context<'a>,
        reply_handle: &poise::ReplyHandle<'a>,
        content: String,
    ) -> Result<(), Error> {
        reply_handle
            .edit(
                ctx,
                poise::CreateReply {
                    content: Some(content),
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }

    /// If the user provided a duration, we want to create
    /// a new pomodoro session with that duration and return it.
    /// If the user did not provide a duration, we want to
    /// create a default session of 60 minutes.
    /// For already existing sessions, return None
    fn _create_session(
        &mut self,
        duration: Option<u64>,
        author_id: serenity::UserId,
    ) -> Option<Session> {
        let session_exists = self.sessions.contains_key(&author_id);
        if session_exists {
            return None;
        }

        let new_session = Session {
            duration: Duration::from_secs(duration.unwrap_or(60) * 60),
            start_time: SystemTime::now(),
        };
        self.sessions.insert(author_id, new_session);

        Some(new_session)
    }

    fn _get_conversation(
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
