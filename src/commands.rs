use crate::handlers::{Command, CommandHandler};
use crate::{Context, Data, Error};

#[poise::command(prefix_command, on_error = "error_handler")]
pub async fn chat(
    ctx: Context<'_>,
    #[description = "The message sent by the user"]
    #[rest]
    message: String,
) -> Result<(), Error> {
    let message = &message;
    let author_id = ctx.author().id;
    let mut command_handler = CommandHandler::new();
    command_handler
        .handle(Command::Chat {
            ctx,
            message,
            author_id,
        })
        .await?;

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
    let mut command_handler = CommandHandler::new();
    command_handler
        .handle(Command::Summarize {
            ctx,
            message,
            author_id,
        })
        .await?;

    Ok(())
}

#[poise::command(
    prefix_command,
    track_edits,
    discard_spare_arguments,
    on_error = "error_handler"
)]
pub async fn session(
    ctx: Context<'_>,
    #[description = "How long the session should last in minutes"] duration: Option<u64>,
) -> Result<(), Error> {
    let author_id = ctx.author().id;
    let mut command_handler = CommandHandler::new();
    command_handler
        .handle(Command::Session {
            ctx,
            duration,
            author_id,
        })
        .await?;

    Ok(())
}

async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("An error occurred: {error}");
}
