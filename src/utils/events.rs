use std::{collections::HashMap, fs, sync::Arc};

use super::{
    data::Data,
    event::{Event, EventBuilder},
};
use serde::{Deserialize, Serialize};
use serenity::{model::prelude::*, prelude::*};
use std::fs::File;

const PATH: &str = "saved_event.json";

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Events(HashMap<ScheduledEventId, Event>);

impl TypeMapKey for Events {
    type Value = Arc<RwLock<Events>>;
}

impl Events {
    async fn get_lock(ctx: &Context) -> Arc<RwLock<Events>> {
        let data_read = ctx.data.read().await;
        data_read
            .get::<Events>()
            .expect("Expected EventsCounter in data.")
            .clone()
    }
    fn write_to_file(&self) {
        let events = self.clone();
        let data = serde_json::to_string_pretty(&events).expect("Serialization failed.");
        fs::write(PATH, data).expect("Can't save data.");
    }
    pub async fn add(ctx: &Context, scheduled_event: ScheduledEvent) {
        let Some(hackathon_channel) = Data::get_hackathon_channel(ctx).await else {
            return;
        };
        let event = EventBuilder::new(&scheduled_event)
            .build_and_send(ctx, hackathon_channel)
            .await
            .unwrap();

        let events_lock = Events::get_lock(ctx).await;

        {
            let mut events = events_lock.write().await;
            events.0.insert(scheduled_event.id, event);
            events.write_to_file();
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
            events.write_to_file();
        }
    }
    pub async fn update(ctx: &Context, scheduled_event: ScheduledEvent) {
        let Some(hackathon_channel) = Data::get_hackathon_channel(ctx).await else {
            return;
        };
        let events_lock = Events::get_lock(ctx).await;
        {
            let mut events = events_lock.write().await;
            if let Some(event) = events.0.get(&scheduled_event.id) {
                let mut event = event.clone();
                event.update(ctx, scheduled_event).await;
                events.0.insert(event.scheduled_event.id, event);
            } else {
                let event = EventBuilder::new(&scheduled_event)
                    .build_and_send(ctx, hackathon_channel)
                    .await
                    .unwrap();
                events.0.insert(event.scheduled_event.id, event);
            };
            events.write_to_file();
        }
    }
    pub async fn refresh(ctx: &Context, ready: &Ready) {
        let guilds = ready.guilds.iter();
        for guild in guilds {
            let events = ctx
                .http
                .get_scheduled_events(guild.id, false)
                .await
                .expect("Cannot get events");
            for event in events {
                Events::update(ctx, event).await;
            }
        }
    }
    pub fn from_file() -> Events {
        match File::open(PATH) {
            Err(_) => Events::default(),
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Err(_) => Events::default(),
                    Ok(events) => events,
                }
            }
        }
    }
}
