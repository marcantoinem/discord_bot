use super::{msg::EventMessage, team::Team};
use serde::{Deserialize, Serialize};
use serenity::{model::prelude::*, prelude::*};

pub const CHANNEL_ID: ChannelId = ChannelId(1050254533537845288);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub teams: Vec<Team>,
    pub event: ScheduledEvent,
    pub msg: Message,
}

impl Event {
    pub async fn update(&mut self, ctx: &Context, scheduled_event: ScheduledEvent) {
        let msg = EventMessage::new().event(&scheduled_event);
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

pub struct EventBuilder {
    teams: Vec<Team>,
    event: Option<ScheduledEvent>,
}

impl EventBuilder {
    pub fn new() -> EventBuilder {
        EventBuilder {
            teams: vec![Team::default()],
            event: None,
        }
    }
    pub fn event(mut self, event: &ScheduledEvent) -> EventBuilder {
        self.event = Some(event.clone());
        self
    }
    pub async fn build_and_send(self, ctx: &Context, channel_id: ChannelId) -> Option<Event> {
        let event = self.event.unwrap();
        let msg = EventMessage::new().event(&event);
        match &msg.build_and_send(ctx, channel_id).await {
            Ok(message) => Some(Event {
                teams: self.teams,
                event,
                msg: message.clone(),
            }),
            Err(why) => {
                println!("Error creating message: {:?}", why);
                None
            }
        }
    }
}
