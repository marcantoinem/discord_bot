use std::num::NonZeroU64;

use serenity::{
    builder::*, collector::ComponentInteractionCollector, model::prelude::*, prelude::*,
};

use crate::utils::prelude::*;

async fn select_team(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<Option<(ComponentInteraction, ScheduledEventId, TeamId)>, serenity::Error> {
    let guild_id = interaction.guild_id.unwrap();
    let Some(menu) = Events::menu_team_with_user(ctx, guild_id, interaction.user.id).await else {
        CreateInteractionResponseMessage::new()
            .content("Veuillez rejoindre une équipe avant d'essayer de quitter une équipe.")
            .build_and_send(ctx, interaction.id, &interaction.token)
            .await?;
        return Ok(None);
    };
    CreateInteractionResponseMessage::new()
        .select_menu(menu)
        .content("Sélectionnez l'équipe que vous voulez quitter.")
        .ephemeral(true)
        .build_and_send(ctx, interaction.id, &interaction.token)
        .await?;
    let interaction = ComponentInteractionCollector::new(&ctx.shard)
        .next()
        .await
        .ok_or(SerenityError::Other("Team selection failed."))?;
    let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind else {
            return Err(SerenityError::Other("Team selection failed."));
    };
    let mut values = values[0].split('/');
    let event_id = ScheduledEventId(values.next().unwrap().parse::<NonZeroU64>().unwrap());
    let team_id = TeamId(values.next().unwrap().parse::<u64>().unwrap());
    Ok(Some((interaction, event_id, team_id)))
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let participant = Interface::new(ctx, interaction.guild_id.unwrap());
    let Some((interaction, event_id, team_id)) = select_team(ctx, interaction).await? else {
        return Ok(())
    };

    let team = participant
        .quit_equip(event_id, team_id, interaction.user)
        .await?;

    CreateInteractionResponseMessage::new()
        .content(format!("Vous avez quitté l'équipe: {}", team.name))
        .components(vec![])
        .build_and_edit(ctx, interaction.id, &interaction.token)
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("quit").description("Quit a team")
}
