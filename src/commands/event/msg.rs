use serenity::{model::prelude::*, prelude::*};

pub struct EventMessage {
    title: String,
    description: String,
    start_time: Timestamp,
    location: String,
    image: Option<String>,
}

impl EventMessage {
    pub fn new() -> EventMessage {
        EventMessage {
            title: String::from(""),
            description: String::from(""),
            start_time: Timestamp::now(),
            location: String::from(""),
            image: None,
        }
    }
    pub fn event(mut self, scheduled_event: &ScheduledEvent) -> EventMessage {
        self.title = scheduled_event.name.clone();
        self.description = scheduled_event.description.clone().unwrap_or_default();
        self.start_time = scheduled_event.start_time;
        if let Some(metadata) = &scheduled_event.metadata {
            self.location = metadata.location.clone();
        }
        if let Some(image_id) = &scheduled_event.image {
            self.image = Some(
                "https://cdn.discordapp.com/guild-events/".to_owned()
                    + &scheduled_event.id.to_string()
                    + "/"
                    + &image_id
                    + "?size=512",
            );
        }
        self
    }
    pub async fn build_and_send(
        &self,
        ctx: &Context,
        channel_id: ChannelId,
    ) -> Result<Message, SerenityError> {
        let title = self.title.clone();
        let description = self.description.clone();
        let location = self.location.clone();
        let start_time = self.start_time.clone();
        if let Some(image) = self.image.clone() {
            channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title(title)
                            .description(description)
                            .image(image)
                            .footer(|f| f.text(location))
                            .timestamp(start_time)
                    })
                })
                .await
        } else {
            channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title(title)
                            .description(description)
                            .footer(|f| f.text(location))
                            .timestamp(start_time)
                    })
                })
                .await
        }
    }
    pub async fn build_and_edit(
        &self,
        ctx: &Context,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Result<Message, SerenityError> {
        let title = self.title.clone();
        let description = self.description.clone();
        let location = self.location.clone();
        let start_time = self.start_time.clone();
        if let Some(image) = self.image.clone() {
            channel_id
                .edit_message(&ctx.http, message_id, |m| {
                    m.embed(|e| {
                        e.title(title)
                            .description(description)
                            .image(image)
                            .footer(|f| f.text(location))
                            .timestamp(start_time)
                    })
                })
                .await
        } else {
            channel_id
                .edit_message(&ctx.http, message_id, |m| {
                    m.embed(|e| {
                        e.title(title)
                            .description(description)
                            .footer(|f| f.text(location))
                            .timestamp(start_time)
                    })
                })
                .await
        }
    }
}
