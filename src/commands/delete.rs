use crate::utils::prelude::*;
use serenity::{
    builder::*, collector::ComponentInteractionCollector, model::prelude::*, prelude::*,
};
use std::num::NonZeroU64;

async fn select_event(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<Option<(ComponentInteraction, ScheduledEventId)>, serenity::Error> {
    let guild_id = interaction.guild_id.unwrap();
    let menu = Events::menu(ctx, guild_id).await;
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
    Ok(Some((interaction, event_id)))
}

async fn select_team(
    ctx: &Context,
    interaction: &ComponentInteraction,
    event_id: &ScheduledEventId,
) -> Result<(ComponentInteraction, TeamId), serenity::Error> {
    let guild_id = interaction.guild_id.unwrap();
    let menu = Teams::menu(ctx, guild_id, *event_id).await;
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
    let interface = Interface::new(ctx, interaction.guild_id.unwrap());
    let Some((interaction, event_id)) = select_event(ctx, interaction).await? else {
        return Ok(())
    };
    let (interaction, team_id) = select_team(ctx, &interaction, &event_id).await?;

    let msg = match interface.delete_equip(event_id, team_id).await {
        Ok(team) => format!("Vous avez détruit l'équipe {}", team.name),
        Err(err) => format!("{}", err),
    };
    CreateInteractionResponseMessage::new()
        .content(msg)
        .components(vec![])
        .build_and_edit(ctx, interaction.id, &interaction.token)
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("delete").description("Delete a team.")
}
