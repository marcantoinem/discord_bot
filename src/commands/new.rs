use std::num::NonZeroU64;

use crate::utils::{events::Events, preference::Preference, traits::SendOrEdit};
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
    let Some(category) = Preference::get_hackathon_category(ctx).await else {
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

    let description = interaction
        .data
        .options
        .get(1)
        .map_or("", |option| option.value.as_str().unwrap());
    let guild_id = interaction
        .guild_id
        .ok_or(SerenityError::Other("Guild creation failed."))?;

    let (interaction, event_id) = select_event(ctx, interaction).await?;
    let mut event = Events::get(ctx, &event_id)
        .await
        .ok_or(SerenityError::Other("Guild creation failed."))?;
    let bot_id = ctx.http.get_current_user().await?.id;
    // Guild id is also the everyone role id.
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(RoleId(NonZeroU64::from(guild_id))),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(bot_id),
        },
    ];
    let text_channel = CreateChannel::new(name)
        .kind(ChannelType::Text)
        .category(category)
        .permissions(permissions.clone())
        .execute(ctx, guild_id)
        .await?;
    let audio_channel = CreateChannel::new(name)
        .kind(ChannelType::Voice)
        .category(category)
        .permissions(permissions)
        .execute(ctx, guild_id)
        .await?;
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
