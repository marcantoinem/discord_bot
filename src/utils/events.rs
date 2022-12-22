use std::{
    collections::{hash_map::Iter, HashMap},
    fs,
    sync::Arc,
};

use super::{
    constants::EVENTS_PATH,
    data::Data,
    event::{Event, EventBuilder},
};
use serde::{Deserialize, Serialize};
use serenity::{builder::*, model::prelude::*, prelude::*};
use std::fs::File;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Events(HashMap<ScheduledEventId, Event>);

impl TypeMapKey for Events {
    type Value = Arc<RwLock<Events>>;
}

/// Private function for events
impl Events {
    fn write_to_file(&self) {
        let events = self.clone();
        let data = serde_json::to_string_pretty(&events).expect("Serialization failed.");
        fs::write(EVENTS_PATH, data).expect("Can't save data.");
    }
    async fn read_events(ctx: &Context) -> Events {
        let events_lock = Events::get_lock(ctx).await;
        let events = events_lock.read().await;
        events.clone()
    }
    async fn get_lock(ctx: &Context) -> Arc<RwLock<Events>> {
        let data_read = ctx.data.read().await;
        data_read
            .get::<Events>()
            .expect("Expected EventsCounter in data.")
            .clone()
    }
    pub fn from_file() -> Events {
        match File::open(EVENTS_PATH) {
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
    pub async fn get(ctx: &Context, id: &ScheduledEventId) -> Option<Event> {
        let events_lock = Events::get_lock(ctx).await;
        let events = events_lock.read().await;
        events.0.get(id).cloned()
    }
    pub fn get_mut(&mut self, id: &ScheduledEventId) -> Option<&mut Event> {
        self.0.get_mut(id)
    }
    pub fn iter(&self) -> Iter<'_, ScheduledEventId, Event> {
        self.0.iter()
    }
}

/// Function which use Events
impl Events {
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
            let Some(event) = events.0.get(&scheduled_event.id) else {
                return;
            };
            if let Err(why) = ctx
                .http
                .delete_message(event.channel_id, event.msg_id, None)
                .await
            {
                println!("An error occurred while running the client: {:?}", why);
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
                let event = event.clone();
                event.update(ctx, hackathon_channel).await;
                events.0.insert(scheduled_event.id, event);
            } else {
                let event = EventBuilder::new(&scheduled_event)
                    .build_and_send(ctx, hackathon_channel)
                    .await
                    .unwrap();
                events.0.insert(scheduled_event.id, event);
            };
            events.write_to_file();
        }
    }
    pub async fn refresh_event(ctx: &Context, event: &Event) {
        let Some(hackathon_channel) = Data::get_hackathon_channel(ctx).await else {
            return;
        };
        let events_lock = Events::get_lock(ctx).await;
        {
            let mut events = events_lock.write().await;
            events.0.entry(event.id).and_modify(|e| *e = event.clone());
            event.update(ctx, hackathon_channel).await;
            events.0.insert(event.id, event.clone());
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
    pub async fn menu(ctx: &Context) -> CreateSelectMenu {
        let events = Events::read_events(ctx).await;
        let options = events
            .iter()
            .map(|(id, event)| CreateSelectMenuOption::new(event.name.clone(), id.to_string()))
            .collect();
        let select_menu = CreateSelectMenuKind::String { options };
        CreateSelectMenu::new("events", select_menu)
    }
    pub async fn menu_nonzero_team(ctx: &Context) -> Option<CreateSelectMenu> {
        let events = Events::read_events(ctx).await;
        let options: Vec<CreateSelectMenuOption> = events
            .iter()
            .filter(|(_, event)| !event.teams.is_empty())
            .map(|(id, event)| CreateSelectMenuOption::new(event.name.clone(), id.to_string()))
            .collect();
        if options.is_empty() {
            return None;
        }
        let select_menu = CreateSelectMenuKind::String { options };
        Some(CreateSelectMenu::new("events", select_menu))
    }
}
