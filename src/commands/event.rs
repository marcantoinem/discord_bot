use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use crate::commands::team::Team;
use serde::{Deserialize, Serialize};
use serenity::{
    // framework::standard::CommandResult,
    // framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};

const CHANNEL_ID: ChannelId = ChannelId(1050254533537845288);
pub const PATH: &str = "./saved_data.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    teams: Vec<Team>,
    event: ScheduledEvent,
    msg: Message,
}

pub struct EventBuilder {
    teams: Vec<Team>,
    event: Option<ScheduledEvent>,
}

impl EventBuilder {
    pub fn new() -> EventBuilder {
        EventBuilder {
            teams: vec![Team::default()],
            event: None,
        }
    }
    pub fn event(mut self, event: ScheduledEvent) -> EventBuilder {
        self.event = Some(event);
        self
    }
    pub async fn build_and_send(self, ctx: &Context, channel: ChannelId) -> Option<Event> {
        let event = self.event.unwrap();
        let msg = MessageBuilder::new()
            .push_bold_line(&event.name.clone())
            .push_line(&event.clone().description.unwrap_or(String::from("")))
            .build();
        match channel.say(&ctx.http, &msg).await {
            Ok(message) => Some(Event {
                teams: self.teams,
                event: event.clone(),
                msg: message,
            }),
            Err(why) => {
                println!("Error creating message: {:?}", why);
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Events(HashMap<ScheduledEventId, Event>);

impl Event {
    pub async fn update(&mut self, ctx: &Context, scheduled_event: ScheduledEvent) {
        let title = scheduled_event.name.clone();
        let description = scheduled_event
            .description
            .clone()
            .unwrap_or(String::from(""));
        let msg = MessageBuilder::new()
            .push_bold_line(title)
            .push_line(description)
            .build();
        match self.msg.edit(ctx, |m| m.content(msg.clone())).await {
            Ok(_) => self.msg.content = msg,
            Err(why) => match CHANNEL_ID.say(&ctx.http, &msg).await {
                Ok(message) => self.msg = message,
                Err(why) => {
                    println!("Error creating message: {:?}", why);
                }
            },
        }
    }
}

impl Events {
    pub async fn add(ctx: &Context, scheduled_event: ScheduledEvent) {
        let event = EventBuilder::new()
            .event(scheduled_event)
            .build_and_send(&ctx, CHANNEL_ID)
            .await
            .unwrap();

        let events_lock = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<EventsContainer>()
                .expect("Expected EventsCounter in data.")
                .clone()
        };

        {
            let mut events = events_lock.write().await;
            events.0.insert(event.event.id, event);
            let serialized_json =
                serde_json::to_string_pretty(&events.0).expect("Serialization failed.");
            fs::write(PATH, serialized_json).expect("Can't save data.");
            println!("{:?}", events.0);
        }
    }
    pub async fn delete(ctx: &Context, scheduled_event: ScheduledEvent) {
        let events_lock = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<EventsContainer>()
                .expect("Expected EventsCounter in data.")
                .clone()
        };

        {
            let mut events = events_lock.write().await;
            if let Some(event) = events.0.get(&scheduled_event.id) {
                if let Err(why) = event.msg.delete(ctx).await {
                    println!("An error occurred while running the client: {:?}", why);
                }
            }
            events.0.remove(&scheduled_event.id);
            let serialized_json =
                serde_json::to_string_pretty(&events.0).expect("Serialization failed.");
            fs::write(PATH, serialized_json).expect("Can't save data.");
            println!("{:?}", events.0);
        }
    }
    pub async fn update(ctx: &Context, scheduled_event: ScheduledEvent) {
        let events_lock = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<EventsContainer>()
                .expect("Expected EventsCounter in data.")
                .clone()
        };

        {
            let mut events = events_lock.write().await;
            let mut event = events.0.get(&scheduled_event.id).unwrap().clone();
            event.update(ctx, scheduled_event).await;
            events.0.insert(event.event.id, event);
            let serialized_json =
                serde_json::to_string_pretty(&events.0).expect("Serialization failed.");
            fs::write(PATH, serialized_json).expect("Can't save data.");
            println!("{:?}", events.0);
        }
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

// #[command]
// pub async fn create_event(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
//     let Ok(title) = args.single::<String>() else {
//             msg.reply(ctx, "Please enter a valid title.").await?;
//             return Ok(());
//         };
//     let Ok(description) = args.single::<String>() else {
//             msg.reply(ctx, "Please enter a valid description.").await?;
//             return Ok(());
//         };
//     let Ok(Channel::Guild(channel)) = msg.channel(ctx).await else {
//             msg.reply(ctx, "An error occured.").await?;
//             return Ok(());
//         };
//     let Some(event) = EventBuilder::new()
//         .title(title)
//         .description(description)
//         .build_and_send(ctx, channel)
//         .await else {
//             msg.reply(ctx, "An error occured.").await?;
//             return Ok(());
//         };

//     event.add(ctx).await;

//     Ok(())
// }
