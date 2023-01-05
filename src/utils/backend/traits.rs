use async_trait::async_trait;
use serenity::{all::InteractionId, builder::*, prelude::*};

type MaybeErr = Result<(), serenity::Error>;

/// Trait for Building and Sending an interaction response message.
#[async_trait]
pub trait SendOrEdit {
    async fn build_and_send(self, ctx: &Context, id: InteractionId, token: &str) -> MaybeErr;
    async fn build_and_edit(self, ctx: &Context, id: InteractionId, token: &str) -> MaybeErr;
}

#[async_trait]
impl SendOrEdit for CreateInteractionResponseMessage {
    async fn build_and_send(self, ctx: &Context, id: InteractionId, token: &str) -> MaybeErr {
        CreateInteractionResponse::Message(self)
            .execute(ctx, id, token)
            .await?;
        Ok(())
    }
    async fn build_and_edit(self, ctx: &Context, id: InteractionId, token: &str) -> MaybeErr {
        CreateInteractionResponse::UpdateMessage(self)
            .execute(ctx, id, token)
            .await?;
        Ok(())
    }
}
