use anyhow::anyhow;
use chatgpt::prelude::ChatGPT;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::error;

// Use when using tokio::main testing.
// use std::env;

mod bot;
mod commands;
use bot::Bot;

async fn new_chat(openai_key: String) -> ChatGPT {
    if let Err(e) = ChatGPT::new(openai_key.clone()) {
        error!("Error getting openai client: {}", e);
    }
    let client = ChatGPT::new(openai_key).unwrap();
    client
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let openai_key = if let Some(key) = secret_store.get("OPENAI_KEY") {
        key
    } else {
        return Err(anyhow!("'OPENAI_KEY' was not found").into());
    };

    let chat_gpt = new_chat(openai_key).await;

    // static GPT_START: bool = false;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let bot = Bot {
        gpt_client: chat_gpt,
    };

    let client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}

// #[tokio::main]
// async fn main() {
//     // Get the discord token set in `Secrets.toml`
//     let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
//     let openai_key = env::var("OPENAI_KEY").expect("Expected a key in the environment");
//
//     let chat_gpt = new_chat(openai_key).await;
//
//     // static GPT_START: bool = false;
//
//     // Set gateway intents, which decides what events the bot will be notified about
//     let intents = GatewayIntents::GUILD_MESSAGES
//         | GatewayIntents::DIRECT_MESSAGES
//         | GatewayIntents::MESSAGE_CONTENT;
//
//     let bot = Bot {
//         gpt_client: chat_gpt,
//     };
//
//     let mut client = Client::builder(&token, intents)
//         .event_handler(bot)
//         .await
//         .expect("Err creating client");
//
//     if let Err(why) = client.start().await {
//             println!("Client error: {:?}", why);
//     }
// }
