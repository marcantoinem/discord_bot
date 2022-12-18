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
    let Some(interaction) = ComponentInteractionCollector::new(&ctx.shard)
        .collect_single()
        .await
        else {
            return Err(SerenityError::Other("Event selection failed."));
        };
    let ComponentInteractionDataKind::StringSelect { values } = interaction.data.kind else {
            return Err(SerenityError::Other("Event selection failed."));
    };
    event_id = ScheduledEventId(values[0].parse::<NonZeroU64>().unwrap());
    let menu = Teams::menu(ctx, event_id).await;
    CreateInteractionResponseMessage::new()
        .content("Sélectionnez l'équipe que vous voulez rejoindre.")
        .components(vec![CreateActionRow::SelectMenu(menu)])
        .build_and_edit(ctx, interaction.id, &interaction.token)
        .await?;

    let Some(interaction) = ComponentInteractionCollector::new(&ctx.shard)
        .collect_single()
        .await else {
            return Err(SerenityError::Other("Event joining failed."));
        };
    let ComponentInteractionDataKind::StringSelect { values } = interaction.data.kind else {
        return Err(SerenityError::Other("Event joining failed."));
    };
    team_id = TeamId(values[0].parse::<u64>().unwrap());
    let Some(mut event) = ({
                let events_lock = Events::get_lock(ctx).await;
                let events = events_lock.read().await;
                events.get(&event_id)
            }) else {
                return Err(SerenityError::Other("Event joining failed."));
            };
    let Some(team) = event.teams.get_team(&team_id) else {
                return Err(SerenityError::Other("Event joining failed."));
            };
    let participant = Participant::from_user(interaction.user);
    println!("{}", event.teams);
    let msg = match event.teams.add_participant(team_id, participant) {
        Ok(_) => format! {"Vous avez été rajouté à l'équipe: {}", team.name},
        Err(error) => format! {"Vous n'avez pas été rajouté à l'équipe: {}", error},
    };
    println!("{}", event.teams);
    Events::refresh_team(ctx, &event).await;
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
