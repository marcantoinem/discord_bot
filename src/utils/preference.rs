use serde::{Deserialize, Serialize};
use serenity::{
    all::{ChannelId, GuildId},
    prelude::*,
};
use std::{
    fs::{self, File},
    path::Path,
    sync::Arc,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Preference {
    pub guild_id: GuildId,
    hackathon_channel: Option<ChannelId>,
    hackathon_category: Option<ChannelId>,
}

impl Preference {
    pub async fn get_lock(ctx: &Context) -> Arc<RwLock<Preference>> {
        let data_read = ctx.data.read().await;
        data_read
            .get::<Preference>()
            .expect("Expected EventsCounter in data.")
            .clone()
    }
    pub fn write_to_file(&self) {
        let data = serde_json::to_string_pretty(&self).expect("Serialization failed.");
        let path = "cache/".to_owned() + &self.guild_id.to_string() + "_saved_preference.json";
        fs::write(path, data).expect("Can't save data.");
    }
    pub async fn get_hackathon_channel(ctx: &Context) -> Option<ChannelId> {
        let lock = Preference::get_lock(ctx).await;
        let read = lock.read().await;
        read.hackathon_channel
    }
    pub async fn edit_hackathon_channel(ctx: &Context, new_hackathon_channel: ChannelId) {
        let lock = Preference::get_lock(ctx).await;
        let mut read = lock.write().await;
        read.hackathon_channel = Some(new_hackathon_channel);
        read.write_to_file();
    }
    pub async fn get_hackathon_category(ctx: &Context) -> Option<ChannelId> {
        let lock = Preference::get_lock(ctx).await;
        let read = lock.read().await;
        read.hackathon_category
    }
    pub async fn edit_hackathon_category(ctx: &Context, new_hackathon_category: ChannelId) {
        let lock = Preference::get_lock(ctx).await;
        let mut read = lock.write().await;
        read.hackathon_category = Some(new_hackathon_category);
        read.write_to_file();
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
