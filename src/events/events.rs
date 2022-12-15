use std::{collections::HashMap, fs, sync::Arc};

use crate::events::event::event::{Event, EventBuilder, CHANNEL_ID};
use serde::{Deserialize, Serialize};
use serenity::{model::prelude::*, prelude::*};

pub const PATH: &str = "./saved_data.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Events(HashMap<ScheduledEventId, Event>);

pub struct EventsContainer;
impl TypeMapKey for EventsContainer {
    type Value = Arc<RwLock<Events>>;
}

impl Events {
    async fn get_lock(ctx: &Context) -> Arc<RwLock<Events>> {
        let data_read = ctx.data.read().await;
        data_read
            .get::<EventsContainer>()
            .expect("Expected EventsCounter in data.")
            .clone()
    }
    pub async fn add(ctx: &Context, scheduled_event: ScheduledEvent) {
        let event = EventBuilder::new(&scheduled_event)
            .build_and_send(&ctx, CHANNEL_ID)
            .await
            .unwrap();

        let events_lock = Events::get_lock(ctx).await;

        {
            let mut events = events_lock.write().await;
            events.0.insert(scheduled_event.id, event);
            let data = serde_json::to_string_pretty(&events.0).expect("Serialization failed.");
            fs::write(PATH, data).expect("Can't save data.");
        }
    }
    pub async fn delete(ctx: &Context, scheduled_event: ScheduledEvent) {
        let events_lock = Events::get_lock(ctx).await;

        {
            let mut events = events_lock.write().await;
            if let Some(event) = events.0.get(&scheduled_event.id) {
                if let Err(why) = event.msg.delete(ctx).await {
                    println!("An error occurred while running the client: {:?}", why);
                }
            }
            events.0.remove(&scheduled_event.id);
            let data = serde_json::to_string_pretty(&events.0).expect("Serialization failed.");
            fs::write(PATH, data).expect("Can't save data.");
        }
    }
    pub async fn update(ctx: &Context, scheduled_event: ScheduledEvent) {
        let events_lock = Events::get_lock(ctx).await;
        {
            let mut events = events_lock.write().await;
            if let Some(event) = events.0.get(&scheduled_event.id).clone() {
                let mut event = event.clone();
                event.update(ctx, scheduled_event).await;
                events.0.insert(event.scheduled_event.id, event);
            } else {
                let event = EventBuilder::new(&scheduled_event)
                    .build_and_send(&ctx, CHANNEL_ID)
                    .await
                    .unwrap();
                events.0.insert(event.scheduled_event.id, event);
            };

            let events = events.clone();
            let data = serde_json::to_string_pretty(&events).expect("Serialization failed.");
            fs::write(PATH, data).expect("Can't save data.");
        }
    }
    pub async fn refresh(ctx: &Context, ready: &Ready) {
        let guilds = ready.guilds.iter();
        for guild in guilds {
            let events = ctx
                .http
                .get_scheduled_events(guild.id, false)
                .await
                .expect("Cannot get event");
            for event in events {
                Events::update(ctx, event).await;
            }
        }
    }
}

impl Default for Events {
    fn default() -> Self {
        Events(HashMap::new())
    }
}
