use std::num::NonZeroU64;

use serenity::{
    builder::*, collector::ComponentInteractionCollector, model::prelude::*, prelude::*,
};

use crate::utils::{
    events::Events,
    team::{TeamId, Teams},
    traits::SendOrEdit,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let menu = Events::menu(ctx).await;
    let event_id;
    let team_id;

    CreateInteractionResponseMessage::new()
        .select_menu(menu)
        .content("Sélectionnez l'événement que vous voulez rejoindre.")
        .ephemeral(true)
        .build_and_send(ctx, interaction.id, &interaction.token)
        .await?;
    if let Some(interaction) = ComponentInteractionCollector::new(&ctx.shard)
        .collect_single()
        .await
    {
        if let ComponentInteractionDataKind::StringSelect { values } = interaction.data.kind {
            event_id = ScheduledEventId(values[0].parse::<NonZeroU64>().unwrap());
            let menu = Teams::menu(ctx, event_id).await;
            CreateInteractionResponseMessage::new()
                .content("Sélectionnez l'équipe que vous voulez rejoindre.")
                .components(vec![CreateActionRow::SelectMenu(menu)])
                .build_and_edit(ctx, interaction.id, &interaction.token)
                .await?;
        } else {
            return Err(SerenityError::Other("Event selection failed."));
        }
    } else {
        return Err(SerenityError::Other("Event selection failed."));
    }

    if let Some(interaction) = ComponentInteractionCollector::new(&ctx.shard)
        .collect_single()
        .await
    {
        if let ComponentInteractionDataKind::StringSelect { values } = interaction.data.kind {
            team_id = TeamId(values[0].parse::<u64>().unwrap());
            let events_lock = Events::get_lock(ctx).await;
            let events = events_lock.write().await;
            if let Some(event) = events.get(&event_id) {
                if let Some(team) = event.teams.get_team(&team_id) {
                    let msg = format! {"Vous avez été rajouté à l'équipe: {}", team.name};
                    CreateInteractionResponseMessage::new()
                        .content(msg)
                        .components(vec![])
                        .build_and_edit(ctx, interaction.id, &interaction.token)
                        .await?;
                } else {
                    return Err(SerenityError::Other("Event joining failed."));
                }
            } else {
                return Err(SerenityError::Other("Event joining failed."));
            }
        } else {
            return Err(SerenityError::Other("Event joining failed."));
        }
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join").description("Join a team.")
}
