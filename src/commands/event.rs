use crate::commands::team::Team;
use serenity::{
    model::prelude::{GuildChannel, Message},
    prelude::Context,
    utils::MessageBuilder,
};

#[derive(Debug)]
pub struct Event {
    title: String,
    description: String,
    teams: Vec<Team>,
    message: Message,
}

pub struct EventBuilder {
    title: Option<String>,
    description: Option<String>,
    teams: Vec<Team>,
}

impl EventBuilder {
    pub fn new() -> EventBuilder {
        EventBuilder {
            title: None,
            description: None,
            teams: vec![Team::default()],
        }
    }
    pub fn title<Text: Into<String>>(mut self, title: Text) -> EventBuilder {
        self.title = Some(title.into());
        self
    }
    pub fn description<Text: Into<String>>(mut self, description: Text) -> EventBuilder {
        self.description = Some(description.into());
        self
    }
    pub async fn build_and_send(self, ctx: &Context, channel: GuildChannel) -> Option<Event> {
        let msg = MessageBuilder::new()
            .push_bold(&self.title.clone().unwrap())
            .push("\n".to_string() + &self.description.clone().unwrap())
            .build();
        match channel.say(&ctx.http, &msg).await {
            Ok(message) => Some(Event {
                title: self.title.clone().unwrap_or_default(),
                description: self.description.clone().unwrap_or_default(),
                teams: self.teams.clone(),
                message: message,
            }),
            Err(why) => {
                println!("Error sending message: {:?}", why);
                None
            }
        }
    }
}

impl Event {}
