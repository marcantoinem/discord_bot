use crate::utils::events::Events;
use serenity::{builder::*, model::prelude::*, prelude::*};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
    if let Some(guild) = interaction.guild_id {
        let events = ctx.http.get_scheduled_events(guild, false).await.unwrap();
        for event in events {
            Events::update(ctx, event).await;
        }
    }
    "Refresh executed with success".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("refresh").description("Pull all event and refresh.")
}
