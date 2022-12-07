use std::collections::HashMap;
use std::sync::Arc;

use crate::commands::team::Team;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Channel::Guild;
use serenity::model::prelude::MessageId;
use serenity::prelude::{RwLock, TypeMapKey};
use serenity::{
    framework::standard::Args,
    model::prelude::{GuildChannel, Message},
    prelude::Context,
    utils::MessageBuilder,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    title: String,
    description: String,
    teams: Vec<Team>,
    message: Message,
}

pub struct EventBuilder {
    title: Option<String>,
    description: Option<String>,
    teams: Vec<Team>,
}

impl EventBuilder {
    pub fn new() -> EventBuilder {
        EventBuilder {
            title: None,
            description: None,
            teams: vec![Team::default()],
        }
    }
    pub fn title<Text: Into<String>>(mut self, title: Text) -> EventBuilder {
        self.title = Some(title.into());
        self
    }
    pub fn description<Text: Into<String>>(mut self, description: Text) -> EventBuilder {
        self.description = Some(description.into());
        self
    }
    pub async fn build_and_send(self, ctx: &Context, channel: GuildChannel) -> Option<Event> {
        let msg = MessageBuilder::new()
            .push_bold(&self.title.clone().unwrap())
            .push("\n".to_string() + &self.description.clone().unwrap())
            .build();
        match channel.say(&ctx.http, &msg).await {
            Ok(message) => Some(Event {
                title: self.title.clone().unwrap_or_default(),
                description: self.description.clone().unwrap_or_default(),
                teams: self.teams.clone(),
                message: message,
            }),
            Err(why) => {
                println!("Error sending message: {:?}", why);
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Events(HashMap<MessageId, Event>);

pub async fn add_event(ctx: &Context, event: Event) {
    let events_lock = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<EventsContainer>()
            .expect("Expected EventsCounter in data.")
            .clone()
    };

    {
        let mut events = events_lock.write().await;
        let message_id = event.message.id;
        events.0.insert(message_id, event);
    }
}

impl Default for Events {
    fn default() -> Self {
        Events(HashMap::new())
    }
}

pub struct EventsContainer;
impl TypeMapKey for EventsContainer {
    type Value = Arc<RwLock<Events>>;
}

#[command]
pub async fn create_event(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let Ok(title) = args.single::<String>() else {
            msg.reply(ctx, "Please enter a valid title.").await?;
            return Ok(());
        };
    let Ok(description) = args.single::<String>() else {
            msg.reply(ctx, "Please enter a valid description.").await?;
            return Ok(());
        };
    let Ok(Guild(channel)) = msg.channel(ctx).await else {
            msg.reply(ctx, "An error occured.").await?;
            return Ok(());
        };
    let Some(event) = EventBuilder::new()
        .title(title)
        .description(description)
        .build_and_send(ctx, channel)
        .await else {
            msg.reply(ctx, "An error occured.").await?;
            return Ok(());
        };

    add_event(ctx, event).await;

    Ok(())
}
