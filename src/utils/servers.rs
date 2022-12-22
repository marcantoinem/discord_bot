use std::{
    collections::HashMap,
    fs,
    sync::{Arc, RwLock},
};

use serenity::{all::GuildId, prelude::*};

use super::{events::Events, preference::Preference};

#[derive(Clone, Default)]
pub struct ServerEvents(HashMap<GuildId, Events>);

#[derive(Clone, Default)]
pub struct ServerPreference(HashMap<GuildId, Preference>);

impl TypeMapKey for ServerEvents {
    type Value = Arc<RwLock<ServerEvents>>;
}

impl TypeMapKey for ServerPreference {
    type Value = Arc<RwLock<ServerPreference>>;
}

fn end_with<T: AsRef<str>>(to_check: T, end: T) {}

impl ServerEvents {
    pub async fn get_lock(ctx: &Context) -> Arc<RwLock<ServerEvents>> {
        let data_read = ctx.data.read().await;
        data_read
            .get::<ServerEvents>()
            .expect("Expected EventsCounter in data.")
            .clone()
    }
    pub fn from_files() -> ServerEvents {
        let mut server_events = ServerEvents::default();
        let paths = fs::read_dir("./cache").unwrap();
        for entry in paths {
            let Ok(entry) = entry else {
                continue;
            };
            let path = entry.path();
            let Some(events) = Events::from_file(path) else {
                continue;
            };
            server_events.0.insert(events.guild_id, events);
        }
        server_events
    }
}

impl ServerPreference {
    pub async fn get_lock(ctx: &Context) -> Arc<RwLock<ServerEvents>> {
        let data_read = ctx.data.read().await;
        data_read
            .get::<ServerEvents>()
            .expect("Expected EventsCounter in data.")
            .clone()
    }
    pub fn from_files() -> ServerPreference {
        let mut server_preference = ServerPreference::default();
        let paths = fs::read_dir("./cache").unwrap();
        for entry in paths {
            let Ok(entry) = entry else {
                continue;
            };
            let path = entry.path();
            let Some(preference) = Preference::from_file(path) else {
                continue;
            };
            server_preference.0.insert(preference.guild_id, preference);
        }
        server_preference
    }
}
