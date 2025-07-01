use crate::handlers::{Command, CommandHandler};
use crate::{Context, Data, Error};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static COMMAND_HANDLER: Lazy<Mutex<CommandHandler>> =
    Lazy::new(|| Mutex::new(CommandHandler::new()));

#[poise::command(prefix_command, on_error = "error_handler")]
pub async fn chat(
    ctx: Context<'_>,
    #[description = "The message sent by the user"]
    #[rest]
    message: String,
) -> Result<(), Error> {
    let message = &message;
    let author_id = ctx.author().id;
    let response = COMMAND_HANDLER
        .lock()
        .await
        .handle(Command::Chat { message, author_id })
        .await?;
    ctx.reply(response).await?;

    Ok(())
}

#[poise::command(prefix_command, on_error = "error_handler")]
pub async fn summarize(
    ctx: Context<'_>,
    #[description = "The message sent by the user"]
    #[rest]
    message: String,
) -> Result<(), Error> {
    let message = &message;
    let author_id = ctx.author().id;
    let response = COMMAND_HANDLER
        .lock()
        .await
        .handle(Command::Summarize { message, author_id })
        .await?;
    ctx.reply(response).await?;

    Ok(())
}

#[poise::command(prefix_command, on_error = "error_handler")]
pub async fn session(
    ctx: Context<'_>,
    #[description = "How long the session should last in minutes"] duration: Option<u64>,
) -> Result<(), Error> {
    let author_id = ctx.author().id;
    let response = COMMAND_HANDLER
        .lock()
        .await
        .handle(Command::Session {
            duration,
            author_id,
        })
        .await?;
    ctx.reply(response).await?;

    Ok(())
}

async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("An error occurred: {error}");
}
