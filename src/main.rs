mod commands;

use commands::event::{Events, EventsContainer, PATH};
use std::{env, fs, sync::Arc};

use serenity::{
    async_trait,
    framework::standard::{macros::group, StandardFramework},
    model::prelude::*,
    prelude::*,
};

#[group]
// #[commands()]
struct General;

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
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";")) // set the bot's prefix to ";"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Initialize the Arc RwLock which keep the data.
    {
        let mut data = client.data.write().await;
        let saved_data = match fs::read_to_string(PATH) {
            Err(_) => Events::default(),
            Ok(file_content) => {
                serde_json::from_str(&file_content).expect("File is probably corrupted.")
            }
        };
        data.insert::<EventsContainer>(Arc::new(RwLock::new(saved_data)));
    }

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
