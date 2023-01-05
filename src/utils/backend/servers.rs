use std::{collections::HashMap, fs, sync::Arc};

use serenity::{
    all::{GuildId, Ready},
    prelude::*,
};

use super::{events::Events, preference::Preference};

#[derive(Clone, Default, Debug)]
pub struct ServerEvents(pub HashMap<GuildId, Events>);

#[derive(Clone, Default, Debug)]
pub struct ServerPreference(pub HashMap<GuildId, Preference>);

impl TypeMapKey for ServerEvents {
    type Value = Arc<RwLock<ServerEvents>>;
}

impl TypeMapKey for ServerPreference {
    type Value = Arc<RwLock<ServerPreference>>;
}

fn end_with<A: AsRef<str>, B: AsRef<str>>(to_check: A, end: B) -> bool {
    let to_check = to_check.as_ref();
    let end = end.as_ref();
    &to_check[(to_check.len() - end.len())..] == end
}

impl ServerEvents {
    pub async fn init(ctx: &Context, ready: &Ready) {
        let server_events_lock = ServerEvents::get_lock(ctx).await;
        {
            let mut server_events = server_events_lock.write().await;
            for guild_id in ready.guilds.iter().map(|guild| guild.id) {
                if !server_events.0.contains_key(&guild_id) {
                    server_events.0.insert(guild_id, Events::new(guild_id));
                    let events = server_events.0.get(&guild_id).unwrap();
                    events.write_to_file();
                }
            }
        }
    }
    pub async fn get_lock(ctx: &Context) -> Arc<RwLock<ServerEvents>> {
        let data_read = ctx.data.read().await;
        data_read
            .get::<ServerEvents>()
            .expect("Expected ServerEvents in data.")
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
            if !end_with(path.to_str().unwrap_or_default(), "_saved_events.json") {
                continue;
            }
            let Some(events) = Events::from_file(path) else {
                continue;
            };
            server_events.0.insert(events.guild_id, events);
        }
        server_events
    }
}

impl ServerPreference {
    pub async fn init(ctx: &Context, ready: &Ready) {
        let server_preference_lock = ServerPreference::get_lock(ctx).await;
        {
            let mut server_preference = server_preference_lock.write().await;
            for guild_id in ready.guilds.iter().map(|guild| guild.id) {
                if !server_preference.0.contains_key(&guild_id) {
                    server_preference
                        .0
                        .insert(guild_id, Preference::new(guild_id));
                    let preference = server_preference.0.get(&guild_id).unwrap();
                    preference.write_to_file();
                }
            }
        }
    }
    pub async fn get_lock(ctx: &Context) -> Arc<RwLock<ServerPreference>> {
        let data_read = ctx.data.read().await;
        data_read
            .get::<ServerPreference>()
            .expect("Expected ServerEvents in data.")
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
            if !end_with(path.to_str().unwrap_or_default(), "_saved_preference.json") {
                continue;
            }
            let Some(preference) = Preference::from_file(path) else {
                continue;
            };
            server_preference.0.insert(preference.guild_id, preference);
        }
        server_preference
    }
}
