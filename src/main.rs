use std::env;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

mod chat;
mod handlers;

use handlers::*;

static mut CHAT_HANDLER: ChatHandler = ChatHandler { context: None };
static mut SUMMARIZE_HANDLER: SummarizeHandler = SummarizeHandler {};

// TODO: - implement
#[command]
async fn rate(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "I rate 9 out of 10.").await?;

    Ok(())
}

// TODO: - implement
#[command]
async fn document(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "I just documented your code. Yes I did.")
        .await?;

    Ok(())
}

#[command]
async fn chat(ctx: &Context, msg: &Message) -> CommandResult {
    let response = unsafe { CHAT_HANDLER.handle(msg).await? };
    msg.reply(ctx, response).await?;

    Ok(())
}

#[command]
async fn summarize(ctx: &Context, msg: &Message) -> CommandResult {
    let response = unsafe { SUMMARIZE_HANDLER.handle(msg).await? };
    msg.reply(ctx, response).await?;

    Ok(())
}

#[group]
#[commands(rate, document, chat, summarize)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    unsafe {
        CHAT_HANDLER = ChatHandler::new();
        SUMMARIZE_HANDLER = SummarizeHandler::new();
    }

    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("token");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
