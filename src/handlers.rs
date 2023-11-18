use crate::chat::message_chatgpt;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

pub async fn handle_summarize_text(msg: &Message) -> CommandResult<String> {
    // TODO: - reply to different kinds of message contexts using different handlers
    // - when !summarize is follwed by text use the handle_summarize_text handler
    // - when !summarize is followed by a link use the handle_summarize_link handler
    // - when !summarize is a reply to a message use the handle_summarize_message handler

    const PROMPT: &str =
        "Please summarize the following text in as much detail as possible.\n\nText: \n";

    let query = msg.content.clone();
    let query = query.lines().skip(1).collect::<Vec<_>>().join("\n");
    let query = format!("{}{}", PROMPT, query);

    println!("Query: \n\n{}", query);

    let chatgpt_response = message_chatgpt(&query).await?;

    println!("\nChatGPT Response: \n\n{}", chatgpt_response);

    Ok(chatgpt_response)
}

pub async fn handle_summarize_message(msg: &Message) -> CommandResult<String> {
    const PROMPT: &str =
        "The following is a message sent in a Discord channel. Please summarize it in as much detail as possible.\n\nText: \n";

    let query = msg.content.clone();
    let query = query.lines().skip(1).collect::<Vec<_>>().join("\n");
    let query = format!("{}{}", PROMPT, query);

    println!("Query: \n\n{}", query);

    let chatgpt_response = message_chatgpt(&query).await?;

    println!("\nChatGPT Response: \n\n{}", chatgpt_response);

    Ok(chatgpt_response)
}
