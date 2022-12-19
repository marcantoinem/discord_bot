//! Event represent an hackathon and the associated teams.

use super::{msg::EventMessage, team::Teams};
use serde::{Deserialize, Serialize};
use serenity::{model::prelude::*, prelude::*};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub teams: Teams,
    pub scheduled_event: ScheduledEvent,
    pub msg: Message,
}

impl Event {
    pub async fn update(&self, ctx: &Context, hackathon_channel: ChannelId) {
        let mut event = self.clone();
        let msg = EventMessage::new(self);
        match &msg
            .build_and_edit(ctx, hackathon_channel, self.msg.id)
            .await
        {
            Ok(msg) => event.msg = msg.clone(),
            Err(_) => match &msg.build_and_send(ctx, hackathon_channel).await {
                Ok(message) => event.msg = message.clone(),
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

impl From<Event> for EventBuilder {
    fn from(event: Event) -> EventBuilder {
        EventBuilder {
            teams: event.teams,
            scheduled_event: event.scheduled_event,
        }
    }
}
