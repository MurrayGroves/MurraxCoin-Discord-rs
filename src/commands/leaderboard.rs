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
        .name("leaderboard")
        .description("Get the top 10 users by balance. Defaults to global, but can be limited to current server.")
        .create_option(|option| {
            option
                .name("global")
                .description("Whether to get the global leaderboard or the current server's leaderboard. Defaults to global.")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
}

pub async fn run(command: &ApplicationCommandInteraction, data: &mut tokio::sync::RwLockWriteGuard<'_, TypeMap>, ctx: &Context) -> Result<FakeEmbed, anyhow::Error> {
    Err(anyhow!("Not implemented"))
}