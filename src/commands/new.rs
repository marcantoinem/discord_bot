use crate::utils::events::Events;
use serenity::{all::CommandOptionType, builder::*, model::prelude::*, prelude::*};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    if let Some(guild) = interaction.guild_id {
        let events = ctx.http.get_scheduled_events(guild, false).await.unwrap();
        for event in events {
            Events::update(ctx, event).await;
        }
    }
    "Team created with success".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("new")
        .description("Create a new team.")
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "name",
            "new team name",
        ))
}
