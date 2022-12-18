use serenity::{
    builder::*, collector::ComponentInteractionCollector, model::prelude::*, prelude::*,
};

use crate::utils::{data::Data, traits::SendOrEdit};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
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
    if let Some(interaction) = ComponentInteractionCollector::new(&ctx.shard)
        .collect_single()
        .await
    {
        if let ComponentInteractionDataKind::ChannelSelect { values } = interaction.data.kind {
            Data::edit_hackathon_channel(ctx, values[0]).await;
            CreateInteractionResponseMessage::new()
                .content("Merci pour votre choix.")
                .components(vec![])
                .build_and_edit(ctx, interaction.id, &interaction.token)
                .await?;
        }
    }
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("setup").description("Initial setup of the bot.")
}
