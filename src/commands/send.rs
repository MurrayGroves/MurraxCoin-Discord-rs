use crate::FakeEmbed;
use crate::TypeMap;
use crate::anyhow;

use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::client::Context;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("send")
        .description("Send MXC to a user or address")
        .create_option(|option| {
            option
                .name("amount")
                .description("How much MXC to send")
                .kind(CommandOptionType::Number)
                .min_number_value(0.0)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("user")
                .description("A Discord user to send to. This or address is required.")
                .kind(CommandOptionType::User)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("address")
                .description("An address to send to. This or user is required.")
                .kind(CommandOptionType::String)
                .required(false)
        })
}

pub async fn run(command: &ApplicationCommandInteraction, data: &mut tokio::sync::RwLockWriteGuard<'_, TypeMap>, ctx: &Context) -> Result<FakeEmbed, anyhow::Error> {
    Err(anyhow!("Not implemented"))
}