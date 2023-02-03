mod commands;

#[macro_use]
mod embed_macros;

use std::env;
use std::io::Write;
use std::collections::HashMap;

use rand::Rng;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::builder::CreateEmbed;
use serenity::model::application::component::ButtonStyle;
use serenity::utils::Colour;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use log::*;
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct FakeEmbed {
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    color: Option<Colour>,
    footer: Option<String>,
    image: Option<String>,
    thumbnail: Option<String>,
    author: Option<String>,
    timestamp: Option<u64>,
    fields: Option<Vec<(String, String, bool)>>,
    buttons: Option<Vec<FakeButton>>
}

#[derive(Debug, Clone)]
pub struct FakeButton {
    label: String,
    style: ButtonStyle,
    url: Option<String>,
    custom_id: Option<String>,
    disabled: bool,
}

pub enum ConfigValue {
    U64(u64),
    RoleId(serenity::model::prelude::RoleId),
    Bool(bool),
    SubredditList(Vec<String>),
}

/// Stores config values required for operation of the downloader
pub struct ConfigStruct {
    _value: HashMap<String, ConfigValue>
}

impl serenity::prelude::TypeMapKey for ConfigStruct {
    type Value = HashMap<String, ConfigValue>;
}

async fn get_command_response(command: &ApplicationCommandInteraction, ctx: &Context) -> Result<FakeEmbed, anyhow::Error> {
    match command.data.name.as_str() {
        "send" => {
            let mut data = ctx.data.write().await;
            debug!("Got lock");
            commands::send::run(&command, &mut data, &ctx).await
        },

        "claim" => {
            let mut data = ctx.data.write().await;
            debug!("Got lock");
            commands::claim::run(&command, &mut data, &ctx).await
        },

        "balance" => {
            let mut data = ctx.data.write().await;
            debug!("Got lock");
            commands::balance::run(&command, &mut data, &ctx).await
        },

        "address" => {
            let mut data = ctx.data.write().await;
            debug!("Got lock");
            commands::address::run(&command, &mut data, &ctx).await
        },

        "leaderboard" => {
            let mut data = ctx.data.write().await;
            debug!("Got lock");
            commands::leaderboard::run(&command, &mut data, &ctx).await
        },

        _ => {
            Err(anyhow!("Unknown command"))?
        }
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            debug!("Received command interaction: {:#?}", command);
            match command.guild_id {
                Some(guild_id) => {
                    info!("{:?} ({:?}) > {:?} ({:?}) : /{} {:?}", guild_id.name(&ctx.cache).unwrap(), guild_id.as_u64(), command.user.name, command.user.id.as_u64(), command.data.name, command.data.options);
                },
                None => {
                    info!("{:?} ({:?}) : /{} {:?}", command.user.name, command.user.id.as_u64(), command.data.name, command.data.options);
                }
            }

            let command_response = get_command_response(&command, &ctx).await;
            let fake_embed = match command_response {
                Ok(ref embed) => embed.clone(),
                Err(ref why) => {
                    let why = why.to_string();
                    let code = rand::thread_rng().gen_range(0..10000);

                    error!("Error code {} getting command response: {:?}", code, why);

                    FakeEmbed {
                        title: Some("An Error Occurred".to_string()),
                        description: Some(format!("Error code: {}\n Error: {}", code, why)),
                        author: None,
                        url: None,
                        color: Some(Colour::from_rgb(255, 0,0)),
                        footer: None,
                        image: None,
                        thumbnail: None,
                        timestamp: None,
                        fields: None,
                        buttons: None
                    }
                }
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    let to_pass = fake_embed.clone();
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(fake_embed_to_message!(to_pass))
                })
                .await
            {
                if format!("{}", why) == "Interaction has already been acknowledged." {
                    command.edit_original_interaction_response(&ctx.http, |response| {
                        debug!("Already sent response, editing instead");
                        let to_pass = fake_embed.clone();
                        response.embed(fake_embed_to_embed!(to_pass));
                        let to_pass = fake_embed.clone();
                        response.components(fake_embed_to_buttons!(to_pass))
                    }).await.unwrap();
                } else {
                    warn!("Cannot respond to slash command: {}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::claim::register(command))
                .create_application_command(|command| commands::balance::register(command))
                .create_application_command(|command| commands::send::register(command))
                .create_application_command(|command| commands::leaderboard::register(command))
                .create_application_command(|command| commands::address::register(command))
        }).await;

        debug!("I created the following global slash commands: {:#?}", commands);
    }
}

#[tokio::main]
async fn main() {
    env_logger::builder()
    .format(|buf, record| {
        writeln!(buf, "{}: {}", record.level(), record.args())
    })
    .init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}