use std::collections::HashMap;

use flume::Sender;
use serenity::all::*;
use tracing::error;

use crate::common::send_event;

use crate::events::{bot::*, EventCollectionBot};

pub(super) struct Handle {
    pub tx: Sender<EventCollectionBot>,
}

#[async_trait]
impl EventHandler for Handle {
    async fn command_permissions_update(&self, ctx: Context, permission: CommandPermissions) {
        send_event!(
            self,
            EventCollectionBot,
            BCommandPermissionsUpdate { ctx, permission }
        );
    }

    async fn auto_moderation_rule_create(&self, ctx: Context, rule: Rule) {
        send_event!(
            self,
            EventCollectionBot,
            BAutoModerationRuleCreate { ctx, rule }
        );
    }

    async fn auto_moderation_rule_update(&self, ctx: Context, rule: Rule) {
        send_event!(
            self,
            EventCollectionBot,
            BAutoModerationRuleUpdate { ctx, rule }
        );
    }

    async fn auto_moderation_rule_delete(&self, ctx: Context, rule: Rule) {
        send_event!(
            self,
            EventCollectionBot,
            BAutoModerationRuleDelete { ctx, rule }
        );
    }

    async fn auto_moderation_action_execution(&self, ctx: Context, execution: ActionExecution) {
        send_event!(
            self,
            EventCollectionBot,
            BAutoModerationActionExecution { ctx, execution }
        );
    }

    #[cfg(feature = "bot_cache")]
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        send_event!(self, EventCollectionBot, BCacheRead { ctx, guilds });
    }

    #[cfg(feature = "bot_cache")]
    async fn shards_ready(&self, ctx: Context, total_shards: u32) {
        send_event!(self, EventCollectionBot, BShardsReady { ctx, total_shards });
    }

    async fn channel_create(&self, ctx: Context, channel: GuildChannel) {
        send_event!(self, EventCollectionBot, BChannelCreate { ctx, channel });
    }

    async fn category_create(&self, ctx: Context, category: GuildChannel) {
        send_event!(self, EventCollectionBot, BCategoryCreate { ctx, category });
    }

    async fn category_delete(&self, ctx: Context, category: GuildChannel) {
        send_event!(self, EventCollectionBot, BCategoryDelete { ctx, category });
    }

    async fn channel_delete(
        &self,
        ctx: Context,
        channel: GuildChannel,
        messages: Option<Vec<Message>>,
    ) {
        send_event!(
            self,
            EventCollectionBot,
            BChannelDelete {
                ctx,
                channel,
                messages
            }
        );
    }

    async fn channel_pins_update(&self, ctx: Context, pin: ChannelPinsUpdateEvent) {
        send_event!(self, EventCollectionBot, BChannelPinUpdate { ctx, pin });
    }

    async fn channel_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {
        send_event!(self, EventCollectionBot, BChannelUpdate { ctx, old, new });
    }

    async fn guild_audit_log_entry_create(
        &self,
        ctx: Context,
        entry: AuditLogEntry,
        guild_id: GuildId,
    ) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildAuditLogEntryCreate {
                ctx,
                entry,
                guild_id
            }
        );
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildBanAddition {
                ctx,
                guild_id,
                banned_user
            }
        );
    }

    async fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildBanRemoval {
                ctx,
                guild_id,
                unbanned_user
            }
        );
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildCreate { ctx, guild, is_new }
        );
    }

    async fn guild_delete(&self, ctx: Context, incomplete: UnavailableGuild, full: Option<Guild>) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildDelete {
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
        send_event!(
            self,
            EventCollectionBot,
            BGuildEmojisUpdate {
                ctx,
                guild_id,
                current_state
            }
        );
    }

    async fn guild_integrations_update(&self, ctx: Context, guild_id: GuildId) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildIntegrationsUpdate { ctx, guild_id }
        );
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildMemberAddition { ctx, new_member }
        );
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data_if_available: Option<Member>,
    ) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildMemberRemoval {
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
        send_event!(
            self,
            EventCollectionBot,
            BGuildMemberUpdate {
                ctx,
                old_if_available,
                new,
                event
            }
        );
    }

    async fn guild_members_chunk(&self, ctx: Context, chunk: GuildMembersChunkEvent) {
        send_event!(self, EventCollectionBot, BGuildMembersChunk { ctx, chunk });
    }

    async fn guild_role_create(&self, ctx: Context, new: Role) {
        send_event!(self, EventCollectionBot, BGuildRoleCreate { ctx, new });
    }

    async fn guild_role_delete(
        &self,
        ctx: Context,
        guild_id: GuildId,
        removed_role_id: RoleId,
        removed_role_data_if_available: Option<Role>,
    ) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildRoleDelete {
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
        send_event!(
            self,
            EventCollectionBot,
            BGuildRoleUpdate {
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
        send_event!(
            self,
            EventCollectionBot,
            BGuildStickersUpdate {
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
        send_event!(
            self,
            EventCollectionBot,
            BGuildUpdate {
                ctx,
                old_data_if_available,
                new_data
            }
        );
    }

    async fn invite_create(&self, ctx: Context, data: InviteCreateEvent) {
        send_event!(self, EventCollectionBot, BInviteCreate { ctx, data });
    }

    async fn invite_delete(&self, ctx: Context, data: InviteDeleteEvent) {
        send_event!(self, EventCollectionBot, BInviteDelete { ctx, data });
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        send_event!(self, EventCollectionBot, BMessage { ctx, new_message });
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        send_event!(
            self,
            EventCollectionBot,
            BMessageDelete {
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
        send_event!(
            self,
            EventCollectionBot,
            BMessageDeleteBulk {
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
        send_event!(
            self,
            EventCollectionBot,
            BMessageUpdate {
                ctx,
                old_if_available,
                new,
                event
            }
        );
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        send_event!(self, EventCollectionBot, BReactionAdd { ctx, add_reaction });
    }

    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        send_event!(
            self,
            EventCollectionBot,
            BReactionRemove {
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
        send_event!(
            self,
            EventCollectionBot,
            BReactionRemoveAll {
                ctx,
                channel_id,
                removed_from_message_id
            }
        );
    }

    async fn reaction_remove_emoji(&self, ctx: Context, removed_reactions: Reaction) {
        send_event!(
            self,
            EventCollectionBot,
            BReactionRemoveEmoji {
                ctx,
                removed_reactions
            }
        );
    }

    async fn presence_update(&self, ctx: Context, new_data: Presence) {
        send_event!(self, EventCollectionBot, BPresenceUpdate { ctx, new_data });
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        send_event!(
            self,
            EventCollectionBot,
            BReadyEvent {
                ctx,
                data_about_bot
            }
        );
    }

    async fn resume(&self, ctx: Context, event: ResumedEvent) {
        send_event!(self, EventCollectionBot, BResume { ctx, event });
    }

    async fn shard_stage_update(&self, ctx: Context, event: ShardStageUpdateEvent) {
        send_event!(self, EventCollectionBot, BShardStageUpdate { ctx, event });
    }

    async fn typing_start(&self, ctx: Context, event: TypingStartEvent) {
        send_event!(self, EventCollectionBot, BTypingStart { ctx, event });
    }

    async fn user_update(&self, ctx: Context, old_data: Option<CurrentUser>, new: CurrentUser) {
        send_event!(self, EventCollectionBot, BUserUpdate { ctx, old_data, new });
    }

    async fn voice_server_update(&self, ctx: Context, event: VoiceServerUpdateEvent) {
        send_event!(self, EventCollectionBot, BVoiceServerUpdate { ctx, event });
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        send_event!(
            self,
            EventCollectionBot,
            BVoiceStateUpdate { ctx, old, new }
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
        send_event!(
            self,
            EventCollectionBot,
            BVoiceChannelStatusUpdate {
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
        send_event!(
            self,
            EventCollectionBot,
            BWebhookUpdate {
                ctx,
                guild_id,
                belongs_to_channel_id
            }
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        send_event!(
            self,
            EventCollectionBot,
            BInteractionCreate { ctx, interaction }
        );
    }

    async fn integration_create(&self, ctx: Context, integration: Integration) {
        send_event!(
            self,
            EventCollectionBot,
            BIntegrationCreate { ctx, integration }
        );
    }

    async fn integration_update(&self, ctx: Context, integration: Integration) {
        send_event!(
            self,
            EventCollectionBot,
            BIntegrationUpdate { ctx, integration }
        );
    }

    async fn stage_instance_create(&self, ctx: Context, stage_instance: StageInstance) {
        send_event!(
            self,
            EventCollectionBot,
            BStageInstanceCreate {
                ctx,
                stage_instance
            }
        );
    }

    async fn stage_instance_update(&self, ctx: Context, stage_instance: StageInstance) {
        send_event!(
            self,
            EventCollectionBot,
            BStageInstanceUpdate {
                ctx,
                stage_instance
            }
        );
    }

    async fn stage_instance_delete(&self, ctx: Context, stage_instance: StageInstance) {
        send_event!(
            self,
            EventCollectionBot,
            BStageInstanceDelete {
                ctx,
                stage_instance
            }
        );
    }

    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
        send_event!(self, EventCollectionBot, BThreadCreate { ctx, thread });
    }

    async fn thread_update(&self, ctx: Context, old: Option<GuildChannel>, new: GuildChannel) {
        send_event!(self, EventCollectionBot, BThreadUpdate { ctx, old, new });
    }

    async fn thread_delete(
        &self,
        ctx: Context,
        thread: PartialGuildChannel,
        full_thread_data: Option<GuildChannel>,
    ) {
        send_event!(
            self,
            EventCollectionBot,
            BThreadDelete {
                ctx,
                thread,
                full_thread_data
            }
        );
    }

    async fn thread_list_sync(&self, ctx: Context, thread_list_sync: ThreadListSyncEvent) {
        send_event!(
            self,
            EventCollectionBot,
            BThreadListSync {
                ctx,
                thread_list_sync
            }
        );
    }

    async fn thread_member_update(&self, ctx: Context, thread_member: ThreadMember) {
        send_event!(
            self,
            EventCollectionBot,
            BThreadMemberUpdate { ctx, thread_member }
        );
    }

    async fn thread_members_update(
        &self,
        ctx: Context,
        thread_members_update: ThreadMembersUpdateEvent,
    ) {
        send_event!(
            self,
            EventCollectionBot,
            BThreadMembersUpdate {
                ctx,
                thread_members_update
            }
        );
    }

    async fn guild_scheduled_event_create(&self, ctx: Context, event: ScheduledEvent) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildScheduledEventCreate { ctx, event }
        );
    }

    async fn guild_scheduled_event_update(&self, ctx: Context, event: ScheduledEvent) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildScheduledEventUpdate { ctx, event }
        );
    }

    async fn guild_scheduled_event_delete(&self, ctx: Context, event: ScheduledEvent) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildScheduledEventDelete { ctx, event }
        );
    }

    async fn guild_scheduled_event_user_add(
        &self,
        ctx: Context,
        subscribed: GuildScheduledEventUserAddEvent,
    ) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildScheduledEventUserAdd { ctx, subscribed }
        );
    }

    async fn guild_scheduled_event_user_remove(
        &self,
        ctx: Context,
        unsubscribed: GuildScheduledEventUserRemoveEvent,
    ) {
        send_event!(
            self,
            EventCollectionBot,
            BGuildScheduledEventUserRemove { ctx, unsubscribed }
        );
    }

    async fn entitlement_create(&self, ctx: Context, entitlement: Entitlement) {
        send_event!(
            self,
            EventCollectionBot,
            BEntitlementCreate { ctx, entitlement }
        );
    }

    async fn entitlement_update(&self, ctx: Context, entitlement: Entitlement) {
        send_event!(
            self,
            EventCollectionBot,
            BEntitlementUpdate { ctx, entitlement }
        );
    }

    async fn entitlement_delete(&self, ctx: Context, entitlement: Entitlement) {
        send_event!(
            self,
            EventCollectionBot,
            BEntitlementDelete { ctx, entitlement }
        );
    }

    async fn poll_vote_add(&self, ctx: Context, event: MessagePollVoteAddEvent) {
        send_event!(self, EventCollectionBot, BPollVoteAdd { ctx, event });
    }

    async fn poll_vote_remove(&self, ctx: Context, event: MessagePollVoteRemoveEvent) {
        send_event!(self, EventCollectionBot, BPollVoteRemove { ctx, event });
    }

    async fn ratelimit(&self, data: RatelimitInfo) {
        send_event!(self, EventCollectionBot, BRateLimit { data });
    }
}
