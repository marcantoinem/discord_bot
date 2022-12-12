use serenity::{builder::*, collector::ModalInteractionCollector, model::prelude::*, prelude::*};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    // let modal = CreateQuickModal::new("About you")
    //     .timeout(std::time::Duration::from_secs(600))
    //     .short_field("First name")
    //     .short_field("Last name")
    //     .paragraph_field("Hobbies and interests");

    // let response = interaction.quick_modal(ctx, modal).await?.unwrap();

    // let inputs = response.inputs;
    // let (first_name, last_name, hobbies) = (&inputs[0], &inputs[1], &inputs[2]);

    // response
    //     .interaction
    //     .create_response(
    //         ctx,
    //         CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
    //             format!("**Name**: {first_name} {last_name}\n\nHobbies and interests: {hobbies}"),
    //         )),
    //     )
    //     .await?;
    // Ok(())
    // println!("I'm here.");
    let select_menu = CreateSelectMenu::new("hello", CreateSelectMenuKind::User);
    let components = vec![CreateActionRow::SelectMenu(select_menu)];
    let modal =
        CreateModal::new(interaction.id.to_string(), "Team creation").components(components);
    CreateInteractionResponse::Modal(modal)
        .execute(ctx, interaction.id, &interaction.token)
        .await?;
    let modal_interaction = ModalInteractionCollector::new(&ctx.shard)
        .custom_ids(vec!["first_button".to_string()])
        .collect_single()
        .await;

    let modal_interaction = match modal_interaction {
        Some(x) => x,
        None => return Err(serenity::Error::Other("No response from the modal.")),
    };
    let mut inputs = modal_interaction.data.components.iter();

    let input = inputs.next().unwrap();

    modal_interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(format!("{:?}", input)),
            ),
        )
        .await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join").description("Join a team.")
}
