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

#[command]
async fn rate(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "I rate 9 out of 10.").await?;

    Ok(())
}

#[command]
async fn chat(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO: - here we want to give the user the option to have a conversation
    //       - the conversation should maintain context between messages and be user-specific
    let mut chat_handler = ChatHandler::new();
    let response = chat_handler.handle(msg).await?;

    msg.reply(ctx, response).await?;

    Ok(())
}

#[command]
async fn summarize(ctx: &Context, msg: &Message) -> CommandResult {
    let summarize_handler = SummarizeHandler::new();
    let response = summarize_handler.handle(msg).await?;

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
