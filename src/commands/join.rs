use std::num::NonZeroU64;

use serenity::{
    builder::*, collector::ComponentInteractionCollector, model::prelude::*, prelude::*,
};

use crate::utils::{
    events::Events,
    participant::Participant,
    team::{TeamId, Teams},
    traits::SendOrEdit,
};

async fn select_event(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(ComponentInteraction, ScheduledEventId), serenity::Error> {
    let Some(menu) = Events::menu_nonzero_team(ctx).await else {
        CreateInteractionResponseMessage::new()
            .content("Veuillez créer une équipe avant d'essayer de rejoindre une équipe.")
            .build_and_send(ctx, interaction.id, &interaction.token)
            .await?;
        return Err(SerenityError::Other("No events with team."));
    };
    CreateInteractionResponseMessage::new()
        .select_menu(menu)
        .content("Sélectionnez l'événement que vous voulez rejoindre.")
        .ephemeral(true)
        .build_and_send(ctx, interaction.id, &interaction.token)
        .await?;
    let interaction = ComponentInteractionCollector::new(&ctx.shard)
        .next()
        .await
        .ok_or(SerenityError::Other("Event selection failed."))?;
    let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind else {
            return Err(SerenityError::Other("Event selection failed."));
    };
    let event_id = ScheduledEventId(values[0].parse::<NonZeroU64>().unwrap());
    Ok((interaction, event_id))
}

async fn select_team(
    ctx: &Context,
    interaction: &ComponentInteraction,
    event_id: &ScheduledEventId,
) -> Result<(ComponentInteraction, TeamId), serenity::Error> {
    let menu = Teams::menu(ctx, *event_id).await;
    CreateInteractionResponseMessage::new()
        .content("Sélectionnez l'équipe que vous voulez rejoindre.")
        .components(vec![CreateActionRow::SelectMenu(menu)])
        .build_and_edit(ctx, interaction.id, &interaction.token)
        .await?;
    let interaction = ComponentInteractionCollector::new(&ctx.shard)
        .next()
        .await
        .ok_or(SerenityError::Other("Team selection failed."))?;
    let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind else {
        return Err(SerenityError::Other("Team selection failed."));
    };
    let team_id = TeamId(values[0].parse::<u64>().unwrap());
    Ok((interaction, team_id))
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let (interaction, event_id) = select_event(ctx, interaction).await?;
    let (interaction, team_id) = select_team(ctx, &interaction, &event_id).await?;
    let mut event = Events::get(ctx, &event_id)
        .await
        .ok_or(SerenityError::Other("Event joining failed."))?;
    let team = event
        .teams
        .get_team(&team_id)
        .ok_or(SerenityError::Other("Event joining failed."))?;
    let participant = Participant::from_user(interaction.user);
    let msg = match event.teams.add_participant(team_id, participant) {
        Ok(_) => format! {"Vous avez été rajouté à l'équipe: {}", team.name},
        Err(error) => format! {"Vous n'avez pas été rajouté à l'équipe: {}", error},
    };
    Events::refresh_event(ctx, &event).await;
    CreateInteractionResponseMessage::new()
        .content(msg)
        .components(vec![])
        .build_and_edit(ctx, interaction.id, &interaction.token)
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join").description("Join a team.")
}
