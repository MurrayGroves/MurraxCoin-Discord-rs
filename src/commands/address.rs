
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
        .name("address")
        .description("Get a user's address. Defaults to yourself.")
        .create_option(|option| {
            option
                .name("user")
                .description("A Discord user to check.")
                .kind(CommandOptionType::User)
                .required(false)
        })
}

pub async fn run(command: &ApplicationCommandInteraction, data: &mut tokio::sync::RwLockWriteGuard<'_, TypeMap>, ctx: &Context) -> Result<FakeEmbed, anyhow::Error> {
    Err(anyhow!("Not implemented"))
}