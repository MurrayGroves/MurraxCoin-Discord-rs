
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
        .name("balance")
        .description("Check a user or address' balance. Defaults to yourself.")
        .create_option(|option| {
            option
                .name("user")
                .description("A Discord user to check.")
                .kind(CommandOptionType::User)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("address")
                .description("An address to check.")
                .kind(CommandOptionType::String)
                .required(false)
        })
}

pub async fn run(command: &ApplicationCommandInteraction, data: &mut tokio::sync::RwLockWriteGuard<'_, TypeMap>, ctx: &Context) -> Result<FakeEmbed, anyhow::Error> {
    Err(anyhow!("Not implemented"))
}