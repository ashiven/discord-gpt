use poise::serenity_prelude as serenity;

mod chat;
mod handlers;

use handlers::*;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

static mut CHAT_HANDLER: ChatHandler = ChatHandler { context: None };
static mut SUMMARIZE_HANDLER: SummarizeHandler = SummarizeHandler {};

#[poise::command(slash_command, prefix_command)]
async fn chat(ctx: Context<'_>, msg: serenity::Message) -> Result<(), Error> {
    let response = unsafe { CHAT_HANDLER.handle(msg.clone()).await? };
    msg.reply(ctx, response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn summarize(ctx: Context<'_>, msg: serenity::Message) -> Result<(), Error> {
    let response = unsafe { SUMMARIZE_HANDLER.handle(msg.clone()).await? };
    msg.reply(ctx, response).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    unsafe {
        CHAT_HANDLER = ChatHandler::new();
        SUMMARIZE_HANDLER = SummarizeHandler::new();
    }

    let token = std::env::var("DISCORD_TOKEN").expect("missing discord token");
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

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
