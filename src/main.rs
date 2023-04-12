use anyhow::anyhow;
use chatgpt::prelude::ChatGPT;
use chatgpt::types::CompletionResponse;
use serenity::async_trait;
use std::env;
// use serenity::builder::CreateApplicationCommandOption;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::{AttachmentType, Message};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

// use serenity::model::prelude::command::CommandOptionType;

// use std::path::Path;
use url::Url;

// use chatgpt::prelude::*;

// use std::fs::File;
// use std::io::Read;

use serenity::model::channel::ReactionType;

mod commands;

struct Bot {
    gpt_client: ChatGPT,
}

static mut GPT_START: bool = false;

#[async_trait]
trait React {
    async fn react_space_invader(&self, ctx: Context, msg: Message);
}

#[async_trait]
impl React for Bot {
    async fn react_space_invader(&self, ctx: Context, msg: Message) {
        let react_unicode = ReactionType::Unicode("👾".to_string());
        if let Err(e) = msg.react(&ctx, react_unicode).await {
            error!("Error sending message: {:?}", e);
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if msg.content == "!gptstart" {
            unsafe {
                GPT_START = true;
            }
            if let Err(e) = msg.channel_id.say(&ctx.http, "GPT-3 started!").await {
                error!("Error sending message: {:?}", e);
            }
        } else if msg.content == "!gptstop" {
            unsafe {
                GPT_START = false;
            }
            if let Err(e) = msg.channel_id.say(&ctx.http, "GPT-3 stopped!").await {
                error!("Error sending message: {:?}", e);
            }
        }
        unsafe {
            if GPT_START == true && msg.content != "!gptstart" && msg.content != "!gptstop" {
                if let Err(e) = msg.channel_id.say(&ctx.http, "GPT-3 waiting!").await {
                    error!("Error sending message: {:?}", e);
                }
                if let Err(e) = self.gpt_client.send_message(msg.content.clone()).await {
                    error!("Error sending message: {:?}", e);
                }
                let response: CompletionResponse = self
                    .gpt_client
                    .send_message(msg.content.clone())
                    .await
                    .unwrap();
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, response.message().content.clone())
                    .await
                {
                    error!("Error sending message: {:?}", e);
                }
            }
        }
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
            return;
        }
        if msg.content == "!react" {
            Self::react_space_invader(&self, ctx, msg).await;
            return;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content: String = match command.data.name.as_str() {
                "hello_world" => commands::hello_world::run(&command.data.options),
                "attachmentinput" => commands::attachmentinput::run(&command.data.options),
                "jpg" => "Later".to_string(),
                _ => "not implemented :(".to_string(),
            };

            let _: () = match command.data.name.as_str() {
                "hello_world" | "attachmentinput" => {
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content(content))
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
                "jpg" => {
                    let a = commands::jpg::run(&command.data.options);
                    let string_slice = a.as_str();
                    // let path = Path::new(string_slice);
                    // let file_path = AttachmentType::Path(path); // Path start looking from the directory where `Cargo.toml` is located.
                    if let Err(e) = Url::parse(string_slice) {
                        error!("Error sending message: {:?}", e);
                    }
                    let url = Url::parse(string_slice).unwrap();
                    let file_url = AttachmentType::Image(url);
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                // .interaction_response_data(|message| message.content(a).add_file(file_path))
                                // .interaction_response_data(|message| message.add_file(file_path))
                                .interaction_response_data(|message| message.add_file(file_url))
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
                _ => println!("Received unknown command interaction"),
            };

            println!("Interaction end")
            // if let Err(why) = command
            //     .create_interaction_response(&ctx.http, |response| {
            //         response
            //             .kind(InteractionResponseType::ChannelMessageWithSource)
            //             .interaction_response_data(|message| message.content(content))
            //     })
            //     .await
            // {
            //     println!("Cannot respond to slash command: {}", why);
            // }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::hello_world::register(command)
        })
        .await;

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::attachmentinput::register(command)
        })
        .await;

        let _ = Command::create_global_application_command(&ctx.http, |command| {
            commands::jpg::register(command)
        })
        .await;
    }
}

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

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
