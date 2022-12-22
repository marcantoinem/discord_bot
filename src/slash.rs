//! Slash commands are registered and handled here.
use std::env;

use crate::commands::{self};
use crate::utils::events::Events;
use serenity::{builder::*, model::prelude::*, prelude::*};

pub async fn ready(ctx: &Context, ready: Ready) {
    Events::refresh(ctx, &ready).await;
    println!("{} is connected!", ready.user.name);

    let guild_id = GuildId::new(
        env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be an integer"),
    );
    let commands = vec![
        commands::refresh::register(),
        commands::join::register(),
        commands::setup::register(),
        commands::new::register(),
    ];
    let commands = guild_id
        .set_application_commands(&ctx.http, commands)
        .await
        .unwrap();
    println!("I now have the following guild slash commands:",);
    commands.iter().for_each(|x| println!("{}", x.name));
}

pub async fn interaction_create(ctx: &Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        println!("Received command interaction: {}", command.data.name);

        let content = match command.data.name.as_str() {
            "refresh" => Some(commands::refresh::run(ctx, &command).await),
            "new" => {
                commands::new::run(ctx, &command).await.unwrap();
                None
            }
            "join" => {
                commands::join::run(ctx, &command).await.unwrap();
                None
            }
            "setup" => {
                commands::setup::run(ctx, &command).await.unwrap();
                None
            }
            _ => Some("not implemented :(".to_string()),
        };

        if let Some(content) = content {
            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}
