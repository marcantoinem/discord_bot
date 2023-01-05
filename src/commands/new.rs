use crate::utils::prelude::*;
use serenity::{
    all::CommandOptionType, builder::*, collector::ComponentInteractionCollector,
    model::prelude::*, prelude::*,
};
use std::num::NonZeroU64;

async fn select_event(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<(ComponentInteraction, ScheduledEventId), serenity::Error> {
    let menu = Events::menu(ctx, interaction.guild_id.unwrap()).await;

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
    let interface = Interface::new(ctx, interaction.guild_id.unwrap());
    let name = interaction.data.options[0]
        .value
        .as_str()
        .ok_or(SerenityError::Other("Team creation failed."))?;

    let description = interaction
        .data
        .options
        .get(1)
        .map_or("", |option| option.value.as_str().unwrap());

    let (interaction, event_id) = select_event(ctx, interaction).await?;

    let msg = match interface.create_equip(event_id, name, description).await {
        Ok(_) => format!("Vous avez créé l'équipe {}", name),
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
