mod commands;

use std::env;
use std::sync::Arc;

use crate::commands::event::EventsContainer;
use crate::commands::event::CREATE_EVENT_COMMAND;
use commands::event::Events;
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::prelude::Message;
use serenity::prelude::*;

#[group]
#[commands(create_event)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, _msg: Message) {
        let data_read = ctx.data.read().await;
        let events = {
            data_read
                .get::<EventsContainer>()
                .expect("Expected EventsCounter in data.")
                .clone()
        };
        println!("{:?}", events);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // Where the data is kept.
    {
        let mut data = client.data.write().await;
        data.insert::<EventsContainer>(Arc::new(RwLock::new(Events::default())));
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
