use crate::utils::prelude::*;
use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::num::NonZeroU64;
pub struct Interface<'a> {
    guild_id: GuildId,
    ctx: &'a Context,
}

impl Interface<'_> {
    pub fn new(ctx: &Context, guild_id: GuildId) -> Interface {
        Interface { guild_id, ctx }
    }
    pub async fn join_equip<U>(
        &self,
        event_id: ScheduledEventId,
        team_id: TeamId,
        user: U,
    ) -> Result<(), serenity::Error>
    where
        U: Into<Participant>,
    {
        let mut event = Events::get(self.ctx, self.guild_id, &event_id)
            .await
            .ok_or(SerenityError::Other("Event joining failed."))?;
        let team = event
            .teams
            .get_team(&team_id)
            .ok_or(SerenityError::Other("Event joining failed."))?;
        let participant: Participant = user.into();
        let permissions = vec![PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(participant.id),
        }];
        let builder = EditChannel::new().permissions(permissions.clone());
        team.text_channel.edit(self.ctx, builder.clone()).await?;
        team.vocal_channel.edit(self.ctx, builder).await?;
        event.teams.add_participant(team_id, participant)?;
        Events::refresh_event(self.ctx, self.guild_id, &event).await;
        Ok(())
    }
    pub async fn create_equip<T>(
        &self,
        event_id: ScheduledEventId,
        name: T,
        description: T,
    ) -> Result<(), serenity::Error>
    where
        T: Into<String> + Clone,
    {
        let mut event = Events::get(self.ctx, self.guild_id, &event_id)
            .await
            .ok_or(SerenityError::Other("Team creation failed."))?;
        let bot_id = self.ctx.http.get_current_user().await?.id;
        // Guild id is also the everyone role id.
        let category = Preference::get_hackathon_category(self.ctx, self.guild_id)
            .await
            .ok_or(SerenityError::Other(
                "Veuillez sélectionner la catégorie avec la commande `/setup`.",
            ))?;
        let permissions = vec![
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::VIEW_CHANNEL,
                kind: PermissionOverwriteType::Role(RoleId(NonZeroU64::from(self.guild_id))),
            },
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(bot_id),
            },
        ];
        let text_channel = CreateChannel::new(name.clone())
            .kind(ChannelType::Text)
            .category(category)
            .permissions(permissions.clone())
            .execute(self.ctx, self.guild_id)
            .await?;
        let audio_channel = CreateChannel::new(name.clone())
            .kind(ChannelType::Voice)
            .category(category)
            .permissions(permissions)
            .execute(self.ctx, self.guild_id)
            .await?;
        event
            .teams
            .add_team(name, description, text_channel.id, audio_channel.id);
        Events::refresh_event(self.ctx, self.guild_id, &event).await;
        Ok(())
    }
}
