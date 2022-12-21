use std::num::NonZeroU64;

use crate::utils::{data::Data, events::Events, traits::SendOrEdit};
use serenity::{
    all::CommandOptionType, builder::*, collector::ComponentInteractionCollector,
    model::prelude::*, prelude::*,
};

async fn select_event(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(ComponentInteraction, ScheduledEventId), serenity::Error> {
    let menu = Events::menu(ctx).await;

    CreateInteractionResponseMessage::new()
        .select_menu(menu)
        .content("Sélectionnez l'événement où vous voulez créer une équipe.")
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

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let Some(category) = Data::get_hackathon_category(ctx).await else {
        CreateInteractionResponseMessage::new()
        .content("Veuillez sélectionner la catégorie avec la commande `/setup`.")
        .build_and_send(ctx, interaction.id, &interaction.token)
        .await?;
        return Ok(())
    };
    let name = interaction.data.options[0]
        .value
        .as_str()
        .ok_or(SerenityError::Other("Event selection failed."))?;

    let description = match interaction.data.options.get(1) {
        Some(option) => option.value.as_str().unwrap(),
        None => "",
    };
    let guild_id = interaction
        .guild_id
        .ok_or(SerenityError::Other("Guild creation failed."))?;

    let (interaction, event_id) = select_event(ctx, interaction).await?;
    let mut event = Events::get(ctx, &event_id)
        .await
        .ok_or(SerenityError::Other("Guild creation failed."))?;
    let text_channel = CreateChannel::new(name)
        .kind(ChannelType::Text)
        .category(category)
        .execute(ctx, guild_id)
        .await?;
    let audio_channel = CreateChannel::new(name)
        .kind(ChannelType::Voice)
        .category(category)
        .execute(ctx, guild_id)
        .await?;
    println!("{:?}", audio_channel);
    event
        .teams
        .add_team(name, description, text_channel.id, audio_channel.id);
    Events::refresh_event(ctx, &event).await;
    let msg = format!("Vous avez créé l'équipe {}", name);
    CreateInteractionResponseMessage::new()
        .content(msg)
        .components(vec![])
        .build_and_edit(ctx, interaction.id, &interaction.token)
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    let title =
        CreateCommandOption::new(CommandOptionType::String, "name", "new team name").required(true);
    let description = CreateCommandOption::new(
        CommandOptionType::String,
        "description",
        "new team description",
    )
    .required(false);

    CreateCommand::new("new")
        .description("Create a new team.")
        .add_option(title)
        .add_option(description)
}
