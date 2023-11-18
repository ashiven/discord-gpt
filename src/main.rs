use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

// import the chat module
mod chat;

// bring the message_chatgpt function into scope
use chat::message_chatgpt;

#[command]
async fn geh(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Who say's I'm geh?").await?;

    Ok(())
}

#[command]
async fn summarize(ctx: &Context, msg: &Message) -> CommandResult {
    const PROMPT: &str =
        "Please summarize the following text in as much detail as possible.\n\nText: \n";

    // extract the content from the message
    let query = msg.content.clone();

    // delete the first line of the message
    let query = query.lines().skip(1).collect::<Vec<_>>().join("\n");

    // prepend the prompt to the query
    let query = format!("{}{}", PROMPT, query);

    println!("Query: \n\n{}", query);

    let chatgpt_response = message_chatgpt(&query).await?;

    println!("\nChatGPT Response: \n\n{}", chatgpt_response);

    msg.reply(ctx, chatgpt_response).await?;

    Ok(())
}

#[group]
#[commands(geh, summarize)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
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
