use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateMessage, EditMessage};
use serenity::{model::prelude::*, prelude::*};

#[derive(Default)]
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
                    + image_id
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
        let embed = CreateEmbed::new()
            .title(self.title.clone())
            .description(self.description.clone())
            .timestamp(self.start_time)
            .footer(CreateEmbedFooter::new(self.location.clone()));
        if let Some(image) = self.image.clone() {
            let embed = embed.image(image);
            let message = CreateMessage::new().add_embed(embed);
            channel_id.send_message(&ctx.http, message).await
        } else {
            let message = CreateMessage::new().add_embed(embed);
            channel_id.send_message(&ctx.http, message).await
        }
    }
    pub async fn build_and_edit(
        &self,
        ctx: &Context,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Result<Message, SerenityError> {
        let embed = CreateEmbed::new()
            .title(self.title.clone())
            .description(self.description.clone())
            .timestamp(self.start_time)
            .footer(CreateEmbedFooter::new(self.location.clone()));
        if let Some(image) = self.image.clone().clone() {
            let embed = embed.image(image);
            let message = EditMessage::new().add_embed(embed);
            channel_id
                .edit_message(&ctx.http, message_id, message)
                .await
        } else {
            let message = EditMessage::new().add_embed(embed);
            channel_id
                .edit_message(&ctx.http, message_id, message)
                .await
        }
    }
}
