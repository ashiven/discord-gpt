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

// global variables for the handlers that will be initialized in main
static mut CHAT_HANDLER: ChatHandler = ChatHandler { context: None };
static mut SUMMARIZE_HANDLER: SummarizeHandler = SummarizeHandler {};

#[command]
async fn rate(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "I rate 9 out of 10.").await?;

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
#[commands(rate, chat, summarize)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    // Initialize the handlers
    unsafe {
        CHAT_HANDLER = ChatHandler::new();
        SUMMARIZE_HANDLER = SummarizeHandler::new();
    }

    // Load the .env file
    dotenv().ok();

    // Get the discord bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");

    // Create a new instance of the framework with a reference to the General group
    // This GENERAL_GROUP gets created above by the #[group] macro for the struct named General
    // Naming the group, for instance, Cool, would create an instance named COOL_GROUP
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    // Create intents for the bot
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the client using the token, intents and framework
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Start the client and print an error if it occurs during startup
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
