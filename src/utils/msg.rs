use super::event::EventBuilder;
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
    pub fn new<E: Into<EventBuilder> + Clone>(event: &E) -> EventMessage {
        let event: EventBuilder = event.clone().into();
        let image = match &event.scheduled_event.image {
            Some(image) => Some(
                "https://cdn.discordapp.com/guild-events/".to_owned()
                    + &event.scheduled_event.id.to_string()
                    + "/"
                    + &image
                    + "?size=512",
            ),
            None => None,
        };
        let title = event.scheduled_event.name.clone();
        let description = event
            .scheduled_event
            .description
            .clone()
            .unwrap_or_default();
        let start_time = event.scheduled_event.start_time;
        let location = event
            .scheduled_event
            .metadata
            .unwrap_or(ScheduledEventMetadata {
                location: "".to_string(),
            })
            .location
            .clone();

        EventMessage {
            title,
            description,
            start_time,
            location,
            image,
        }
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
