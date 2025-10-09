use std::collections::HashMap;

use flume::Sender;
use serenity::all::*;
use tracing::error;

use crate::common::send_message;

use crate::messages::{MessageCollectionBot, bot::*};

pub(super) struct Handle {
    pub tx: Sender<MessageCollectionBot>,
}

#[async_trait]
impl EventHandler for Handle {
    async fn command_permissions_update(&self, ctx: Context, permission: CommandPermissions) {
        send_message!(
            self,
            MessageCollectionBot,
            CommandPermissionsUpdateMessage { ctx, permission }
        );
    }

    async fn auto_moderation_rule_create(&self, ctx: Context, rule: Rule) {
        send_message!(
            self,
            MessageCollectionBot,
            AutoModerationRuleCreateMessage { ctx, rule }
        );
    }

    async fn auto_moderation_rule_update(&self, ctx: Context, rule: Rule) {
        send_message!(
            self,
            MessageCollectionBot,
            AutoModerationRuleUpdateMessage { ctx, rule }
        );
    }

    async fn auto_moderation_rule_delete(&self, ctx: Context, rule: Rule) {
        send_message!(
            self,
            MessageCollectionBot,
            AutoModerationRuleDeleteMessage { ctx, rule }
        );
    }

    async fn auto_moderation_action_execution(&self, ctx: Context, execution: ActionExecution) {
        send_message!(
            self,
            MessageCollectionBot,
            AutoModerationActionExecutionMessage { ctx, execution }
        );
    }

    #[cfg(feature = "bot_cache")]
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        send_message!(self, MessageCollectionBot, CacheReadMessage { ctx, guilds });
    }

    #[cfg(feature = "bot_cache")]
    async fn shards_ready(&self, ctx: Context, total_shards: u32) {
        send_message!(
            self,
            MessageCollectionBot,
            ShardsReadyMessage { ctx, total_shards }
        );
    }

    async fn channel_create(&self, ctx: Context, channel: GuildChannel) {
        send_message!(
            self,
            MessageCollectionBot,
            ChannelCreateMessage { ctx, channel }
        );
    }

    async fn category_create(&self, ctx: Context, category: GuildChannel) {
        send_message!(
            self,
            MessageCollectionBot,
            CategoryCreateMessage { ctx, category }
        );
    }

    async fn category_delete(&self, ctx: Context, category: GuildChannel) {
        send_message!(
            self,
            MessageCollectionBot,
            CategoryDeleteMessage { ctx, category }
        );
    }

    async fn channel_delete(
        &self,
        ctx: Context,
        channel: GuildChannel,
        messages: Option<Vec<Message>>,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            ChannelDeleteMessage {
                ctx,
                channel,
                messages
            }
        );
    }

    async fn channel_pins_update(&self, ctx: Context, pin: ChannelPinsUpdateEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            ChannelPinUpdateMessage { ctx, pin }
        );
    }

    async fn channel_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {
        send_message!(
            self,
            MessageCollectionBot,
            ChannelUpdateMessage { ctx, old, new }
        );
    }

    async fn guild_audit_log_entry_create(
        &self,
        ctx: Context,
        entry: AuditLogEntry,
        guild_id: GuildId,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildAuditLogEntryCreateMessage {
                ctx,
                entry,
                guild_id
            }
        );
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildBanAdditionMessage {
                ctx,
                guild_id,
                banned_user
            }
        );
    }

    async fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildBanRemovalMessage {
                ctx,
                guild_id,
                unbanned_user
            }
        );
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildCreateMessage { ctx, guild, is_new }
        );
    }

    async fn guild_delete(&self, ctx: Context, incomplete: UnavailableGuild, full: Option<Guild>) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildDeleteMessage {
                ctx,
                incomplete,
                full
            }
        );
    }

    async fn guild_emojis_update(
        &self,
        ctx: Context,
        guild_id: GuildId,
        current_state: HashMap<EmojiId, Emoji>,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildEmojisUpdateMessage {
                ctx,
                guild_id,
                current_state
            }
        );
    }

    async fn guild_integrations_update(&self, ctx: Context, guild_id: GuildId) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildIntegrationsUpdateMessage { ctx, guild_id }
        );
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildMemberAdditionMessage { ctx, new_member }
        );
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data_if_available: Option<Member>,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildMemberRemovalMessage {
                ctx,
                guild_id,
                user,
                member_data_if_available
            }
        );
    }

    async fn guild_member_update(
        &self,
        ctx: Context,
        old_if_available: Option<Member>,
        new: Option<Member>,
        event: GuildMemberUpdateEvent,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildMemberUpdateMessage {
                ctx,
                old_if_available,
                new,
                event
            }
        );
    }

    async fn guild_members_chunk(&self, ctx: Context, chunk: GuildMembersChunkEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildMembersChunkMessage { ctx, chunk }
        );
    }

    async fn guild_role_create(&self, ctx: Context, new: Role) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildRoleCreateMessage { ctx, new }
        );
    }

    async fn guild_role_delete(
        &self,
        ctx: Context,
        guild_id: GuildId,
        removed_role_id: RoleId,
        removed_role_data_if_available: Option<Role>,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildRoleDeleteMessage {
                ctx,
                guild_id,
                removed_role_id,
                removed_role_data_if_available
            }
        );
    }

    async fn guild_role_update(
        &self,
        ctx: Context,
        old_data_if_available: Option<Role>,
        new: Role,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildRoleUpdateMessage {
                ctx,
                old_data_if_available,
                new
            }
        );
    }

    async fn guild_stickers_update(
        &self,
        ctx: Context,
        guild_id: GuildId,
        current_state: HashMap<StickerId, Sticker>,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildStickersUpdateMessage {
                ctx,
                guild_id,
                current_state
            }
        );
    }

    async fn guild_update(
        &self,
        ctx: Context,
        old_data_if_available: Option<Guild>,
        new_data: PartialGuild,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildUpdateMessage {
                ctx,
                old_data_if_available,
                new_data
            }
        );
    }

    async fn invite_create(&self, ctx: Context, data: InviteCreateEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            InviteCreateMessage { ctx, data }
        );
    }

    async fn invite_delete(&self, ctx: Context, data: InviteDeleteEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            InviteDeleteMessage { ctx, data }
        );
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        send_message!(
            self,
            MessageCollectionBot,
            DiscordMessage { ctx, new_message }
        );
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            DiscordMessageDeleteMessage {
                ctx,
                channel_id,
                deleted_message_id,
                guild_id
            }
        );
    }

    async fn message_delete_bulk(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        multiple_deleted_messages_ids: Vec<MessageId>,
        guild_id: Option<GuildId>,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            DiscordMessageDeleteBulkMessage {
                ctx,
                channel_id,
                multiple_deleted_messages_ids,
                guild_id
            }
        );
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            DiscordMessageUpdateMessage {
                ctx,
                old_if_available,
                new,
                event
            }
        );
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        send_message!(
            self,
            MessageCollectionBot,
            ReactionAddMessage { ctx, add_reaction }
        );
    }

    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        send_message!(
            self,
            MessageCollectionBot,
            ReactionRemoveMessage {
                ctx,
                removed_reaction
            }
        );
    }

    async fn reaction_remove_all(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        removed_from_message_id: MessageId,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            ReactionRemoveAllMessage {
                ctx,
                channel_id,
                removed_from_message_id
            }
        );
    }

    async fn reaction_remove_emoji(&self, ctx: Context, removed_reactions: Reaction) {
        send_message!(
            self,
            MessageCollectionBot,
            ReactionRemoveEmojiMessage {
                ctx,
                removed_reactions
            }
        );
    }

    async fn presence_update(&self, ctx: Context, new_data: Presence) {
        send_message!(
            self,
            MessageCollectionBot,
            PresenceUpdateMessage { ctx, new_data }
        );
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        send_message!(
            self,
            MessageCollectionBot,
            BotReadyMessage {
                ctx,
                data_about_bot
            }
        );
    }

    async fn resume(&self, ctx: Context, event: ResumedEvent) {
        send_message!(self, MessageCollectionBot, ResumeMessage { ctx, event });
    }

    async fn shard_stage_update(&self, ctx: Context, event: ShardStageUpdateEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            ShardStageUpdateMessage { ctx, event }
        );
    }

    async fn typing_start(&self, ctx: Context, event: TypingStartEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            TypingStartMessage { ctx, event }
        );
    }

    async fn user_update(&self, ctx: Context, old_data: Option<CurrentUser>, new: CurrentUser) {
        send_message!(
            self,
            MessageCollectionBot,
            UserUpdateMessage { ctx, old_data, new }
        );
    }

    async fn voice_server_update(&self, ctx: Context, event: VoiceServerUpdateEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            VoiceServerUpdateMessage { ctx, event }
        );
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        send_message!(
            self,
            MessageCollectionBot,
            VoiceStateUpdateMessage { ctx, old, new }
        );
    }

    async fn voice_channel_status_update(
        &self,
        ctx: Context,
        old: Option<String>,
        status: Option<String>,
        id: ChannelId,
        guild_id: GuildId,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            VoiceChannelStatusUpdateMessage {
                ctx,
                old,
                status,
                id,
                guild_id
            }
        );
    }

    async fn webhook_update(
        &self,
        ctx: Context,
        guild_id: GuildId,
        belongs_to_channel_id: ChannelId,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            WebhookUpdateMessage {
                ctx,
                guild_id,
                belongs_to_channel_id
            }
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        send_message!(
            self,
            MessageCollectionBot,
            InteractionCreateMessage { ctx, interaction }
        );
    }

    async fn integration_create(&self, ctx: Context, integration: Integration) {
        send_message!(
            self,
            MessageCollectionBot,
            IntegrationCreateMessage { ctx, integration }
        );
    }

    async fn integration_update(&self, ctx: Context, integration: Integration) {
        send_message!(
            self,
            MessageCollectionBot,
            IntegrationUpdateMessage { ctx, integration }
        );
    }

    async fn stage_instance_create(&self, ctx: Context, stage_instance: StageInstance) {
        send_message!(
            self,
            MessageCollectionBot,
            StageInstanceCreateMessage {
                ctx,
                stage_instance
            }
        );
    }

    async fn stage_instance_update(&self, ctx: Context, stage_instance: StageInstance) {
        send_message!(
            self,
            MessageCollectionBot,
            StageInstanceUpdateMessage {
                ctx,
                stage_instance
            }
        );
    }

    async fn stage_instance_delete(&self, ctx: Context, stage_instance: StageInstance) {
        send_message!(
            self,
            MessageCollectionBot,
            StageInstanceDeleteMessage {
                ctx,
                stage_instance
            }
        );
    }

    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
        send_message!(
            self,
            MessageCollectionBot,
            ThreadCreateMessage { ctx, thread }
        );
    }

    async fn thread_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {
        send_message!(
            self,
            MessageCollectionBot,
            ThreadUpdateMessage { ctx, old, new }
        );
    }

    async fn thread_delete(
        &self,
        ctx: Context,
        thread: PartialGuildChannel,
        full_thread_data: Option<GuildChannel>,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            ThreadDeleteMessage {
                ctx,
                thread,
                full_thread_data
            }
        );
    }

    async fn thread_list_sync(&self, ctx: Context, thread_list_sync: ThreadListSyncEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            ThreadListSyncMessage {
                ctx,
                thread_list_sync
            }
        );
    }

    async fn thread_member_update(&self, ctx: Context, thread_member: ThreadMember) {
        send_message!(
            self,
            MessageCollectionBot,
            ThreadMemberUpdateMessage { ctx, thread_member }
        );
    }

    async fn thread_members_update(
        &self,
        ctx: Context,
        thread_members_update: ThreadMembersUpdateEvent,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            ThreadMembersUpdateMessage {
                ctx,
                thread_members_update
            }
        );
    }

    async fn guild_scheduled_event_create(&self, ctx: Context, event: ScheduledEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildScheduledEventCreateMessage { ctx, event }
        );
    }

    async fn guild_scheduled_event_update(&self, ctx: Context, event: ScheduledEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildScheduledEventUpdateMessage { ctx, event }
        );
    }

    async fn guild_scheduled_event_delete(&self, ctx: Context, event: ScheduledEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildScheduledEventDeleteMessage { ctx, event }
        );
    }

    async fn guild_scheduled_event_user_add(
        &self,
        ctx: Context,
        subscribed: GuildScheduledEventUserAddEvent,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildScheduledEventUserAddMessage { ctx, subscribed }
        );
    }

    async fn guild_scheduled_event_user_remove(
        &self,
        ctx: Context,
        unsubscribed: GuildScheduledEventUserRemoveEvent,
    ) {
        send_message!(
            self,
            MessageCollectionBot,
            GuildScheduledEventUserRemoveMessage { ctx, unsubscribed }
        );
    }

    async fn entitlement_create(&self, ctx: Context, entitlement: Entitlement) {
        send_message!(
            self,
            MessageCollectionBot,
            EntitlementCreateMessage { ctx, entitlement }
        );
    }

    async fn entitlement_update(&self, ctx: Context, entitlement: Entitlement) {
        send_message!(
            self,
            MessageCollectionBot,
            EntitlementUpdateMessage { ctx, entitlement }
        );
    }

    async fn entitlement_delete(&self, ctx: Context, entitlement: Entitlement) {
        send_message!(
            self,
            MessageCollectionBot,
            EntitlementDeleteMessage { ctx, entitlement }
        );
    }

    async fn poll_vote_add(&self, ctx: Context, event: MessagePollVoteAddEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            PollVoteAddMessage { ctx, event }
        );
    }

    async fn poll_vote_remove(&self, ctx: Context, event: MessagePollVoteRemoveEvent) {
        send_message!(
            self,
            MessageCollectionBot,
            PollVoteRemoveMessage { ctx, event }
        );
    }

    async fn ratelimit(&self, data: RatelimitInfo) {
        send_message!(self, MessageCollectionBot, RateLimitMessage { data });
    }
}
