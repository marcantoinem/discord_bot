use serde::{Deserialize, Serialize};
use serenity::{
    all::{ChannelId, GuildId, Ready},
    prelude::*,
};
use std::{
    fs::{self, File},
    path::Path,
    sync::Arc,
};
use tokio::sync::RwLock;

use super::servers::ServerPreference;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Preference {
    pub guild_id: GuildId,
    hackathon_channel: Option<ChannelId>,
    hackathon_category: Option<ChannelId>,
}

impl Preference {
    pub async fn init(ctx: &Context, ready: &Ready) {
        let guilds = ready.guilds.iter();
        for guild in guilds {
            let server_events_lock = ServerPreference::get_lock(ctx).await;
            {
                let mut server_events = server_events_lock.write().await;
                if !server_events.0.contains_key(&guild.id) {
                    server_events.0.insert(guild.id, Preference::new(guild.id));
                }
            }
        }
    }
    pub fn new(guild_id: GuildId) -> Preference {
        Preference {
            guild_id,
            hackathon_channel: None,
            hackathon_category: None,
        }
    }
    pub fn write_to_file(&self) {
        let data = serde_json::to_string_pretty(&self).expect("Serialization failed.");
        let path = "cache/".to_owned() + &self.guild_id.to_string() + "_saved_preference.json";
        fs::write(path, data).expect("Can't save data.");
    }
    pub async fn get_hackathon_channel(ctx: &Context, guild_id: GuildId) -> Option<ChannelId> {
        let lock = ServerPreference::get_lock(ctx).await;
        let read = lock.read().await;
        let preference = read.0.get(&guild_id)?;
        preference.hackathon_channel
    }
    pub async fn edit_hackathon_channel(
        ctx: &Context,
        new_hackathon_channel: ChannelId,
        guild_id: GuildId,
    ) {
        let lock = ServerPreference::get_lock(ctx).await;
        let mut read = lock.write().await;
        let mut new_events = Preference::new(guild_id);
        let preference = match read.0.get_mut(&guild_id) {
            Some(events) => events,
            None => &mut new_events,
        };
        preference.hackathon_channel = Some(new_hackathon_channel);
        preference.write_to_file();
    }
    pub async fn get_hackathon_category(ctx: &Context, guild_id: GuildId) -> Option<ChannelId> {
        let lock = ServerPreference::get_lock(ctx).await;
        let read = lock.read().await;
        let preference = read.0.get(&guild_id)?;
        preference.hackathon_category
    }
    pub async fn edit_hackathon_category(
        ctx: &Context,
        new_hackathon_category: ChannelId,
        guild_id: GuildId,
    ) {
        let lock = ServerPreference::get_lock(ctx).await;
        let mut read = lock.write().await;
        let mut new_events = Preference::new(guild_id);
        let preference = match read.0.get_mut(&guild_id) {
            Some(events) => events,
            None => &mut new_events,
        };
        preference.hackathon_category = Some(new_hackathon_category);
        preference.write_to_file();
    }
    pub fn from_file<T: AsRef<Path>>(path: T) -> Option<Preference> {
        match File::open(path) {
            Err(_) => None,
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Err(_) => None,
                    Ok(preference) => Some(preference),
                }
            }
        }
    }
}

impl TypeMapKey for Preference {
    type Value = Arc<RwLock<Preference>>;
}
