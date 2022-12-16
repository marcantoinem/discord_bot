//! Event represent an hackathon and the associated teams.

use super::{msg::EventMessage, team::Teams};
use serde::{Deserialize, Serialize};
use serenity::{model::prelude::*, prelude::*};

pub const CHANNEL_ID: ChannelId = ChannelId::new(1050254533537845288);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub teams: Teams,
    pub scheduled_event: ScheduledEvent,
    pub msg: Message,
}

impl Event {
    pub async fn update(&mut self, ctx: &Context, scheduled_event: ScheduledEvent) {
        self.scheduled_event = scheduled_event;
        let msg = EventMessage::new(self);
        match &msg.build_and_edit(ctx, CHANNEL_ID, self.msg.id).await {
            Ok(msg) => self.msg = msg.clone(),
            Err(_) => match &msg.build_and_send(ctx, CHANNEL_ID).await {
                Ok(message) => self.msg = message.clone(),
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
    pub scheduled_event: ScheduledEvent,
}

impl EventBuilder {
    pub fn new(scheduled_event: &ScheduledEvent) -> EventBuilder {
        EventBuilder {
            teams: Teams::default(),
            scheduled_event: scheduled_event.clone(),
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
                scheduled_event: self.scheduled_event,
                msg: message.clone(),
            }),
            Err(why) => {
                println!("Error creating message: {:?}", why);
                None
            }
        }
    }
}

impl Into<EventBuilder> for Event {
    fn into(self) -> EventBuilder {
        EventBuilder {
            teams: self.teams,
            scheduled_event: self.scheduled_event,
        }
    }
}
