use crate::chat::new_conversation;
use crate::{Context, Error};
use chatgpt::converse::Conversation;
use once_cell::sync::Lazy;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::Mutex;

static COMMAND_HANDLER_STATE: Lazy<Mutex<CommandHandlerState>> = Lazy::new(|| {
    Mutex::new(CommandHandlerState {
        conversations: HashMap::new(),
        sessions: HashMap::new(),
    })
});

pub enum Command<'a> {
    Chat {
        ctx: Context<'a>,
        message: &'a str,
        author_id: serenity::UserId,
    },
    Summarize {
        ctx: Context<'a>,
        message: &'a str,
        author_id: serenity::UserId,
    },
    Session {
        ctx: Context<'a>,
        duration: Option<u64>,
        goals: Option<String>,
        author_id: serenity::UserId,
    },
}

struct CommandHandlerState {
    pub conversations: HashMap<serenity::UserId, Conversation>,
    pub sessions: HashMap<serenity::UserId, Session>,
}

#[derive(Debug, Clone, Copy)]
struct Session {
    pub duration: Duration,
}

pub struct CommandHandler {}

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler {}
    }

    pub async fn handle(&mut self, command: Command<'_>) -> Result<(), Error> {
        match command {
            Command::Chat {
                ctx,
                message,
                author_id,
            } => self.chat(ctx, message, author_id).await?,
            Command::Summarize {
                ctx,
                message,
                author_id,
            } => self.summarize(ctx, message, author_id).await?,
            Command::Session {
                ctx,
                duration,
                goals,
                author_id,
            } => self.session(ctx, duration, goals, author_id).await?,
        };

        Ok(())
    }

    async fn chat(
        &mut self,
        ctx: Context<'_>,
        message: &str,
        author_id: serenity::UserId,
    ) -> Result<(), Error> {
        let response = self._message_gpt(author_id, message).await?;
        self._reply(ctx, response).await?;

        Ok(())
    }

    async fn summarize(
        &mut self,
        ctx: Context<'_>,
        message: &str,
        author_id: serenity::UserId,
    ) -> Result<(), Error> {
        const SUMMARIZE_PROMPT: &str =
            "Please summarize the following text in as much detail as possible. \
            \n\nText: \n";
        let summarize_message = format!("{SUMMARIZE_PROMPT}{message}");
        let response = self._message_gpt(author_id, &summarize_message).await?;
        self._reply(ctx, response).await?;

        Ok(())
    }

    /// If the user provided a duration, we want to create
    /// a new pomodoro session with that duration and return it.
    /// If the user did not provide a duration, we want to
    /// create a default session of 60 minutes.
    /// For already existing sessions, return None
    async fn _create_session(
        &mut self,
        duration: Option<u64>,
        author_id: serenity::UserId,
    ) -> Option<Session> {
        let session_exists = COMMAND_HANDLER_STATE
            .lock()
            .await
            .sessions
            .contains_key(&author_id);
        if session_exists {
            return None;
        }

        let new_session = Session {
            duration: Duration::from_secs(duration.unwrap_or(60) * 60),
        };
        COMMAND_HANDLER_STATE
            .lock()
            .await
            .sessions
            .insert(author_id, new_session);

        Some(new_session)
    }

    async fn _end_session(
        &mut self,
        ctx: Context<'_>,
        reply_handle: &poise::ReplyHandle<'_>,
        session_duration: u64,
        goals: Option<String>,
        author_id: serenity::UserId,
    ) -> Result<(), Error> {
        COMMAND_HANDLER_STATE
            .lock()
            .await
            .sessions
            .remove(&author_id);

        self._edit_reply(ctx, reply_handle, "~Session Completed~".into())
            .await?;

        let mut end_session_prompt = format!(
        "I have just completed a pomodoro session that lasted \
        for {session_duration} minutes. Please inform me in a creative way that my session has ended \
        and that I can take a break now. You may use emojis in your response and include a link \
        to a funny or cute animal video / gif / image or whatever you like.\n"
    );
        if let Some(goals) = goals {
            let goal_assistance_prompt = format!("I had the following goals for this session: '{goals}'. \
            Please ask me whether I achieved them or not and if not, whether I would like some assistance to achieve them in the next session.\n");
            end_session_prompt = format!("{end_session_prompt}\n\n{goal_assistance_prompt}");
        }
        let response = self._message_gpt(author_id, &end_session_prompt).await?;
        self._reply(ctx, response).await?;

        Ok(())
    }

    async fn session(
        &mut self,
        ctx: Context<'_>,
        duration: Option<u64>,
        goals: Option<String>,
        author_id: serenity::UserId,
    ) -> Result<(), Error> {
        let session = self._create_session(duration, author_id).await;
        if session.is_none() {
            let session_active_text = "You already have an active session.".into();
            self._reply(ctx, session_active_text).await?;
            return Ok(());
        }

        let session = session.unwrap();
        let session_duration_minutes = session.duration.as_secs() / 60;
        let start_text =
            format!("Starting your {session_duration_minutes} minute pomodoro session...\n");
        let reply_handle = self._reply(ctx, start_text).await?;

        let mut runtime_seconds_total = 0;
        while runtime_seconds_total < session.duration.as_secs() {
            // TODO: Add a check for whether the session has been cancelled
            let runtime_minutes = runtime_seconds_total / 60;
            let runtime_seconds = runtime_seconds_total % 60;
            let runtime_text = format!(
                "Your {session_duration_minutes} minute session has been running for: \n\n {runtime_minutes}m : {runtime_seconds}s"
            );
            self._edit_reply(ctx, &reply_handle, runtime_text).await?;
            tokio::time::sleep(Duration::from_secs(1)).await;
            runtime_seconds_total += 1;
        }

        self._end_session(
            ctx,
            &reply_handle,
            session_duration_minutes,
            goals,
            author_id,
        )
        .await?;
        Ok(())
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

    async fn _message_gpt(
        &mut self,
        author_id: serenity::UserId,
        message: &str,
    ) -> Result<String, Error> {
        let response = COMMAND_HANDLER_STATE
            .lock()
            .await
            .conversations
            .entry(author_id)
            .or_insert_with(new_conversation)
            .send_message(message)
            .await?
            .message()
            .content
            .clone();

        Ok(response)
    }
}
