//! Event represent an hackathon and the associated teams.

use super::{msg::EventMessage, team::Teams};
use serde::{Deserialize, Serialize};
use serenity::{model::prelude::*, prelude::*};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub teams: Teams,
    pub id: ScheduledEventId,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub start_time: Timestamp,
    pub location: String,
    pub msg_id: MessageId,
    pub channel_id: ChannelId,
}

impl Event {
    pub async fn update(&self, ctx: &Context, hackathon_channel: ChannelId) {
        let mut event = self.clone();
        let msg = EventMessage::new(self);
        match &msg
            .build_and_edit(ctx, hackathon_channel, self.msg_id)
            .await
        {
            Ok(msg) => {
                event.msg_id = msg.id;
                event.channel_id = msg.channel_id
            }
            Err(_) => match &msg.build_and_send(ctx, hackathon_channel).await {
                Ok(msg) => {
                    event.msg_id = msg.id;
                    event.channel_id = msg.channel_id
                }
                Err(why) => {
                    println!("Error creating message: {:?}", why);
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventBuilder {
    pub teams: Teams,
    pub id: ScheduledEventId,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub start_time: Timestamp,
    pub location: String,
}

impl EventBuilder {
    pub fn new(scheduled_event: &ScheduledEvent) -> EventBuilder {
        let image = match &scheduled_event.image {
            Some(image) => Some(
                "https://cdn.discordapp.com/guild-events/".to_owned()
                    + &scheduled_event.id.to_string()
                    + "/"
                    + image
                    + "?size=512",
            ),
            None => None,
        };
        let location = scheduled_event
            .metadata
            .clone()
            .unwrap_or(ScheduledEventMetadata {
                location: "".to_string(),
            })
            .location;
        let description = scheduled_event.description.clone().unwrap_or_default();
        let start_time = scheduled_event.start_time;
        EventBuilder {
            teams: Teams::default(),
            name: scheduled_event.name.clone(),
            description,
            image,
            location,
            start_time,
            id: scheduled_event.id.clone(),
        }
    }
    pub fn teams(mut self, teams: Teams) -> EventBuilder {
        self.teams = teams;
        self
    }
    pub async fn build_and_send(self, ctx: &Context, channel_id: ChannelId) -> Option<Event> {
        let msg = EventMessage::new(&self);
        match &msg.build_and_send(ctx, channel_id).await {
            Ok(message) => Some(Event {
                teams: self.teams,
                id: self.id,
                name: self.name,
                description: self.description,
                image: self.image,
                start_time: self.start_time,
                location: self.location,
                msg_id: message.id,
                channel_id: message.channel_id,
            }),
            Err(why) => {
                println!("Error creating message: {:?}", why);
                None
            }
        }
    }
}

impl From<Event> for EventBuilder {
    fn from(event: Event) -> EventBuilder {
        EventBuilder {
            teams: event.teams,
            id: event.id,
            name: event.name,
            description: event.description,
            image: event.image,
            start_time: event.start_time,
            location: event.location,
        }
    }
}
