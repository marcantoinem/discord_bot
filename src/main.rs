//!
pub mod commands;
pub mod slash;
pub mod utils;

use serenity::{async_trait, model::prelude::*, prelude::*};
use std::{env, sync::Arc};
use utils::{events::Events, servers::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_scheduled_event_create(&self, ctx: Context, scheduled_event: ScheduledEvent) {
        Events::add(&ctx, scheduled_event).await;
    }
    async fn guild_scheduled_event_delete(&self, ctx: Context, scheduled_event: ScheduledEvent) {
        Events::delete(&ctx, scheduled_event).await;
    }
    async fn guild_scheduled_event_update(&self, ctx: Context, scheduled_event: ScheduledEvent) {
        Events::update(&ctx, scheduled_event).await;
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        slash::ready(&ctx, ready).await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        slash::interaction_create(&ctx, interaction).await;
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILDS;
    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Initialize the Arc RwLock which keep the data and refresh it.
    {
        let saved_events = ServerEvents::from_files();
        let saved_data = ServerPreference::from_files();
        let mut data = client.data.write().await;
        data.insert::<ServerEvents>(Arc::new(RwLock::new(saved_events)));
        data.insert::<ServerPreference>(Arc::new(RwLock::new(saved_data)));
    }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
