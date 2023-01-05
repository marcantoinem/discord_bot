use crate::utils::prelude::*;
use serenity::{
    builder::*, collector::ComponentInteractionCollector, model::prelude::*, prelude::*,
};

async fn select_hackathon_channel(
    ctx: &Context,
    interaction: &CommandInteraction,
) -> Result<ComponentInteraction, serenity::Error> {
    let select_menu = CreateSelectMenuKind::Channel {
        channel_types: Some(vec![ChannelType::Text, ChannelType::News]),
    };
    let select_menu = CreateSelectMenu::new("hackathon_channel", select_menu);
    CreateInteractionResponseMessage::new()
        .select_menu(select_menu)
        .content("Sélectionnez le salons où les événements seront envoyés.")
        .ephemeral(true)
        .build_and_send(ctx, interaction.id, &interaction.token)
        .await?;
    let Some(interaction) = ComponentInteractionCollector::new(&ctx.shard)
        .next()
        .await else {
            return Err(SerenityError::Other("Failed to collect interaction for hackathon channel."));
        };
    let ComponentInteractionDataKind::ChannelSelect { values } = &interaction.data.kind else {
            return Err(SerenityError::Other("Failed to collect interaction for hackathon channel."));
        };
    Preference::edit_hackathon_channel(ctx, values[0], interaction.guild_id.unwrap()).await;
    Ok(interaction)
}

async fn select_hackathon_category(
    ctx: &Context,
    interaction: &ComponentInteraction,
) -> Result<ComponentInteraction, serenity::Error> {
    let select_menu = CreateSelectMenuKind::Channel {
        channel_types: Some(vec![ChannelType::Category]),
    };
    let select_menu = CreateSelectMenu::new("hackathon_channel", select_menu);
    CreateInteractionResponseMessage::new()
        .select_menu(select_menu)
        .content("Sélectionnez la catégorie où les équipes auront leurs salons.")
        .ephemeral(true)
        .build_and_edit(ctx, interaction.id, &interaction.token)
        .await?;
    let Some(interaction) = ComponentInteractionCollector::new(&ctx.shard)
        .next()
        .await else {
            return Err(SerenityError::Other("Failed to collect interaction for hackathon channel."));
        };
    let ComponentInteractionDataKind::ChannelSelect { values } = &interaction.data.kind else {
            return Err(SerenityError::Other("Failed to collect interaction for hackathon channel."));
        };
    Preference::edit_hackathon_category(ctx, values[0], interaction.guild_id.unwrap()).await;
    Ok(interaction)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let interaction = select_hackathon_channel(ctx, interaction).await?;
    let interaction = select_hackathon_category(ctx, &interaction).await?;
    CreateInteractionResponseMessage::new()
        .content("Merci pour votre choix.")
        .components(vec![])
        .build_and_edit(ctx, interaction.id, &interaction.token)
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("setup").description("Initial setup of the bot.")
}
