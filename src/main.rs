use dotenv::dotenv;
use once_cell::sync::Lazy;
use poise::serenity_prelude as serenity;
use tokio::sync::Mutex;

mod chat;
mod handlers;

use handlers::*;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

static CHAT_HANDLER: Lazy<Mutex<ChatHandler>> = Lazy::new(|| Mutex::new(ChatHandler::new()));
static SUMMARIZE_HANDLER: Lazy<Mutex<SummarizeHandler>> =
    Lazy::new(|| Mutex::new(SummarizeHandler::new()));

#[poise::command(slash_command, prefix_command)]
async fn chat(ctx: Context<'_>, msg: String) -> Result<(), Error> {
    let message = &msg;
    let author_id = ctx.author().id;
    let response = CHAT_HANDLER.lock().await.handle(message, author_id).await?;
    ctx.reply(response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn summarize(ctx: Context<'_>, msg: String) -> Result<(), Error> {
    let message = &msg;
    let response = SUMMARIZE_HANDLER.lock().await.handle(message).await?;
    ctx.reply(response).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let discord_token = std::env::var("DISCORD_TOKEN").expect("missing discord token");

    let intents = serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![chat(), summarize()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
