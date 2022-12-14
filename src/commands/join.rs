use serenity::{
    builder::*, collector::ComponentInteractionCollector, model::prelude::*, prelude::*,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let select_menu = CreateSelectMenu::new("hello", CreateSelectMenuKind::User);
    let message = CreateInteractionResponseMessage::new()
        .select_menu(select_menu)
        .content("Je suis un retour.")
        .ephemeral(true);
    CreateInteractionResponse::Message(message)
        .execute(ctx, interaction.id, &interaction.token)
        .await?;
    if let Some(interaction) = ComponentInteractionCollector::new(&ctx.shard)
        .collect_single()
        .await
    {
        println!("{:?}", interaction.message.id);
        let response = CreateInteractionResponseMessage::new()
            .content("Thank's for your response.")
            .components(vec![]);
        CreateInteractionResponse::UpdateMessage(response)
            .execute(ctx, interaction.id, &interaction.token)
            .await?;
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join").description("Join a team.")
}
