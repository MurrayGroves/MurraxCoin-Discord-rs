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
        .name("claim")
        .description("Claim your daily MXC")
}

pub async fn run(command: &ApplicationCommandInteraction, data: &mut tokio::sync::RwLockWriteGuard<'_, TypeMap>, ctx: &Context) -> Result<FakeEmbed, anyhow::Error> {
    Err(anyhow!("Not implemented"))
}