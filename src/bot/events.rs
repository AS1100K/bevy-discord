//! This module contains all the bevy events that are send
//!
//! NOTE: Every Event has a prefix `B` to avoid confusion with `serenity` events.

use bevy_ecs::prelude::*;
use serenity::all::*;
use std::collections::HashMap;

#[derive(Event)]
/// Dispatched upon startup.
///
/// Provides data about the bot and the guilds it’s in.
pub struct BReadyEvent {
    pub ctx: Context,
    pub data_about_bot: Ready,
}

#[derive(Event)]
/// Dispatched when the permissions of an application command was updated.
///
/// Provides said permission’s data.
pub struct BCommandPermissionsUpdate {
    pub ctx: Context,
    pub permission: CommandPermissions,
}

#[derive(Event)]
/// Dispatched when an auto moderation rule was created.
///
/// Provides said rule’s data.
pub struct BAutoModerationRuleCreate {
    pub ctx: Context,
    pub rule: Rule,
}

#[derive(Event)]
/// Dispatched when an auto moderation rule was updated.
///
/// Provides said rule’s data.
pub struct BAutoModerationRuleUpdate {
    pub ctx: Context,
    pub rule: Rule,
}

#[derive(Event)]
/// Dispatched when an auto moderation rule was deleted.
///
/// Provides said rule’s data.
pub struct BAutoModerationRuleDelete {
    pub ctx: Context,
    pub rule: Rule,
}

#[derive(Event)]
/// Dispatched when an auto moderation rule was triggered and an action was executed.
///
/// Provides said action execution’s data.
pub struct BAutoModerationActionExecution {
    pub ctx: Context,
    pub execution: ActionExecution,
}

#[cfg(feature = "bot_cache")]
#[cfg_attr(docsrs, doc(cfg(feature = "bot_cache")))]
#[derive(Event)]
/// Dispatched when the cache has received and inserted all data from guilds.
///
/// This process happens upon starting your bot and should be fairly quick. However, cache actions
/// performed prior this event may fail as the data could be not inserted yet.
///
/// Provides the cached guilds’ ids.
pub struct BCacheRead {
    pub ctx: Context,
    pub guilds: Vec<GuildId>,
}

#[cfg(feature = "bot_cache")]
#[cfg_attr(docsrs, doc(cfg(feature = "bot_cache")))]
#[derive(Event)]
/// Dispatched when every shard has received a Ready event
pub struct BShardsReady {
    pub ctx: Context,
    pub total_shards: u32,
}

#[derive(Event)]
/// Dispatched when a channel is created.
///
/// Provides said channel’s data.
pub struct BChannelCreate {
    pub ctx: Context,
    pub channel: GuildChannel,
}

#[derive(Event)]
/// Dispatched when a category is created.
///
/// Provides said category’s data.
pub struct BCategoryCreate {
    pub ctx: Context,
    pub category: GuildChannel,
}

#[derive(Event)]
/// Dispatched when a category is deleted.
///
/// Provides said category’s data.
pub struct BCategoryDelete {
    pub ctx: Context,
    pub category: GuildChannel,
}

#[derive(Event)]
/// Dispatched when a channel is deleted.
///
/// Provides said channel’s data.
pub struct BChannelDelete {
    pub ctx: Context,
    pub channel: GuildChannel,
    pub messages: Option<Vec<Message>>,
}

#[derive(Event)]
/// Dispatched when a pin is added, deleted.
///
/// Provides said pin’s data.
pub struct BChannelPinUpdate {
    pub ctx: Context,
    pub pin: ChannelPinsUpdateEvent,
}

#[derive(Event)]
/// Dispatched when a channel is updated.
///
/// The old channel data is only provided when the `bot_cache` feature is enabled.
pub struct BChannelUpdate {
    pub ctx: Context,
    pub old: Option<GuildChannel>,
    pub new: GuildChannel,
}

#[derive(Event)]
/// Dispatched when a new audit log entry is created.
///
/// Provides said entry’s data and the id of the guild where it was created.
pub struct BGuildAuditLogEntryCreate {
    pub ctx: Context,
    pub entry: AuditLogEntry,
    pub guild_id: GuildId,
}

#[derive(Event)]
/// Dispatched when a user is banned from a guild.
///
/// Provides the guild’s id and the banned user’s data.
pub struct BGuildBanAddition {
    pub ctx: Context,
    pub guild_id: GuildId,
    pub banned_user: User,
}

#[derive(Event)]
/// Dispatched when a user’s ban is lifted from a guild.
///
/// Provides the guild’s id and the lifted user’s data.
pub struct BGuildBanRemoval {
    pub ctx: Context,
    pub guild_id: GuildId,
    pub unbanned_user: User,
}

#[derive(Event)]
/// Dispatched when a guild is created; or an existing guild’s data is sent to us.
///
/// Provides the guild’s data and whether the guild is new (only when `bot_cache` feature is enabled).
pub struct BGuildCreate {
    pub ctx: Context,
    pub guild: Guild,
    pub is_new: Option<bool>,
}

#[derive(Event)]
/// Dispatched when a guild is deleted.
///
/// Provides the partial data of the guild sent by discord, and the full data from the cache,
/// if `bot_cache` feature is enabled and the data is available.
///
/// The [`UnavailableGuild::unavailable`] flag in the partial data determines the status of the guild. If the flag
/// is false, the bot was removed from the guild, either by being kicked or banned. If the
/// flag is true, the guild went offline.
pub struct BGuildDelete {
    pub ctx: Context,
    pub incomplete: UnavailableGuild,
    pub full: Option<Guild>,
}

#[derive(Event)]
/// Dispatched when the emojis are updated.
///
/// Provides the guild’s id and the new state of the emojis in the guild.
pub struct BGuildEmojisUpdate {
    pub ctx: Context,
    pub guild_id: GuildId,
    pub current_state: HashMap<EmojiId, Emoji>,
}

#[derive(Event)]
/// Dispatched when a guild’s integration is added, updated or removed.
///
/// Provides the guild’s id.
pub struct BGuildIntegrationsUpdate {
    pub ctx: Context,
    pub guild_id: GuildId,
}

#[derive(Event)]
/// Dispatched when a user joins a guild.
///
/// Provides the guild’s id and the user’s member data.
///
/// Note: This event will not trigger unless the “guild members” privileged intent is enabled
/// on the bot application page.
pub struct BGuildMemberAddition {
    pub ctx: Context,
    pub new_member: Member,
}

#[derive(Event)]
/// Dispatched when a user’s membership ends by leaving, getting kicked, or being banned.
///
/// Provides the guild’s id, the user’s data, and the user’s member data if cache feature is
/// enabled and the data is available.
///
/// Note: This event will not trigger unless the “guild members” privileged intent is enabled
/// on the bot application page.
pub struct BGuildMemberRemoval {
    pub ctx: Context,
    pub guild_id: GuildId,
    pub user: User,
    pub member_data_if_available: Option<Member>,
}

#[derive(Event)]
/// Dispatched when a member is updated (e.g their nickname is updated).
///
/// Provides the member’s old and new data (if cache feature is enabled and data is available)
/// and the new raw data about updated fields.
///
/// Note: This event will not trigger unless the “guild members” privileged intent is enabled
/// on the bot application page.
pub struct BGuildMemberUpdate {
    pub ctx: Context,
    pub old_if_available: Option<Member>,
    pub new: Option<Member>,
    pub event: GuildMemberUpdateEvent,
}

#[derive(Event)]
/// Dispatched when the data for offline members was requested.
///
/// Provides the guild’s id and the data.
pub struct BGuildMembersChunk {
    pub ctx: Context,
    pub chunk: GuildMembersChunkEvent,
}

#[derive(Event)]
/// Dispatched when a role is created.
///
/// Provides the guild’s id and the new role’s data.
pub struct BGuildRoleCreate {
    pub ctx: Context,
    pub new: Role,
}

#[derive(Event)]
/// Dispatched when a role is deleted.
/// Provides the guild’s id, the role’s id and its data (if `bot_cache` feature is enabled
/// and the data is available).
pub struct BGuildRoleDelete {
    pub ctx: Context,
    pub guild_id: GuildId,
    pub removed_role_id: RoleId,
    pub removed_role_data_if_available: Option<Role>,
}

#[derive(Event)]
/// Dispatched when a role is updated.
///
/// Provides the guild’s id, the role’s old (if `bot_cache` feature is enabled and the data
/// is available) and new data.
pub struct BGuildRoleUpdate {
    pub ctx: Context,
    pub old_data_if_available: Option<Role>,
    pub new: Role,
}

#[derive(Event)]
/// Dispatched when the stickers are updated.
///
/// Provides the guild’s id and the new state of the stickers in the guild.
pub struct BGuildStickersUpdate {
    pub ctx: Context,
    pub guild_id: GuildId,
    pub current_state: HashMap<StickerId, Sticker>,
}

#[derive(Event)]
/// Dispatched when the guild is updated.
///
/// Provides the guild’s old data (if `bot_cache` feature is enabled and the data is
/// available) and the new data.
pub struct BGuildUpdate {
    pub ctx: Context,
    pub old_data_if_available: Option<Guild>,
    pub new_data: PartialGuild,
}

#[derive(Event)]
/// Dispatched when a invite is created.
///
/// Provides data about the invite.
pub struct BInviteCreate {
    pub ctx: Context,
    pub data: InviteCreateEvent,
}

#[derive(Event)]
/// Dispatched when a invite is deleted.
///
/// Provides data about the invite.
pub struct BInviteDelete {
    pub ctx: Context,
    pub data: InviteDeleteEvent,
}

#[derive(Event)]
/// Dispatched when a message is created.
///
/// Provides the message’s data.
pub struct BMessage {
    pub ctx: Context,
    pub new_message: Message,
}

#[derive(Event)]
/// Dispatched when a message is deleted.
///
/// Provides the guild’s id, the channel’s id and the message’s id.
pub struct BMessageDelete {
    pub ctx: Context,
    pub channel_id: ChannelId,
    pub deleted_message_id: MessageId,
    pub guild_id: Option<GuildId>,
}

#[derive(Event)]
/// Dispatched when multiple messages were deleted at once.
///
/// Provides the guild’s id, channel’s id and the deleted messages’ ids.
pub struct BMessageDeleteBulk {
    pub ctx: Context,
    pub channel_id: ChannelId,
    pub multiple_deleted_messages_ids: Vec<MessageId>,
    pub guild_id: Option<GuildId>,
}

#[derive(Event)]
/// Dispatched when a message is updated.
///
/// Provides the message update data, as well as the actual old and new message if `bot_cache`
/// feature is enabled and the data is available.
pub struct BMessageUpdate {
    pub ctx: Context,
    pub old_if_available: Option<Message>,
    pub new: Option<Message>,
    pub event: MessageUpdateEvent,
}

#[derive(Event)]
/// Dispatched when a new reaction is attached to a message.
///
/// Provides the reaction’s data.
pub struct BReactionAdd {
    pub ctx: Context,
    pub add_reaction: Reaction,
}

#[derive(Event)]
/// Dispatched when a reaction is detached from a message.
///
/// Provides the reaction’s data.
pub struct BReactionRemove {
    pub ctx: Context,
    pub removed_reaction: Reaction,
}

#[derive(Event)]
/// Dispatched when all reactions of a message are detached from a message.
///
/// Provides the channel’s id and the message’s id.
pub struct BReactionRemoveAll {
    pub ctx: Context,
    pub channel_id: ChannelId,
    pub removed_from_message_id: MessageId,
}

#[derive(Event)]
/// Dispatched when all reactions of a message are detached from a message.
///
/// Provides the channel’s id and the message’s id.
pub struct BReactionRemoveEmoji {
    pub ctx: Context,
    pub removed_reactions: Reaction,
}

#[derive(Event)]
/// Dispatched when a user’s presence is updated (e.g off -> on).
///
/// Provides the presence’s new data.
///
/// Note: This event will not trigger unless the “guild presences” privileged intent is enabled
/// on the bot application page.
pub struct BPresenceUpdate {
    pub ctx: Context,
    pub new_data: Presence,
}

#[derive(Event)]
/// Dispatched upon reconnection.
pub struct BResume {
    pub ctx: Context,
    pub event: ResumedEvent,
}

#[derive(Event)]
/// Dispatched when a shard’s connection stage is updated
///
/// Provides the context of the shard and the event information about the update.
pub struct BShardStageUpdate {
    pub ctx: Context,
    pub event: ShardStageUpdateEvent,
}

#[derive(Event)]
/// Dispatched when a user starts typing.
pub struct BTypingStart {
    pub ctx: Context,
    pub event: TypingStartEvent,
}

#[derive(Event)]
/// Dispatched when the bot’s data is updated.
///
/// Provides the old (if `bot_cache` feature is enabled and the data is available) and new data.
pub struct BUserUpdate {
    pub ctx: Context,
    pub old_data: Option<CurrentUser>,
    pub new: CurrentUser,
}

#[derive(Event)]
/// Dispatched when a guild’s voice server was updated (or changed to another one).
///
/// Provides the voice server’s data.
pub struct BVoiceServerUpdate {
    pub ctx: Context,
    pub event: VoiceServerUpdateEvent,
}

#[derive(Event)]
/// Dispatched when a user joins, leaves or moves to a voice channel.
///
/// Provides the guild’s id (if available) and the old state (if `bot_cache` feature is enabled
/// and [`GatewayIntents::GUILDS`] is enabled) and the new state of the guild’s voice channels.
pub struct BVoiceStateUpdate {
    pub ctx: Context,
    pub old: Option<VoiceState>,
    pub new: VoiceState,
}

#[derive(Event)]
/// Dispatched when a voice channel’s status is updated.
///
/// Provides the status, channel’s id and the guild’s id.
pub struct BVoiceChannelStatusUpdate {
    pub ctx: Context,
    pub old: Option<String>,
    pub status: Option<String>,
    pub id: ChannelId,
    pub guild_id: GuildId,
}

#[derive(Event)]
/// Dispatched when a guild’s webhook is updated.
///
/// Provides the guild’s id and the channel’s id the webhook belongs in.
pub struct BWebhookUpdate {
    pub ctx: Context,
    pub guild_id: GuildId,
    pub belongs_to_channel_id: ChannelId,
}

#[derive(Event)]
/// Dispatched when an interaction is created (e.g a slash command was used or a button was
/// clicked).
///
/// Provides the created interaction.
pub struct BInteractionCreate {
    pub ctx: Context,
    pub interaction: Interaction,
}

#[derive(Event)]
/// Dispatched when a guild integration is created.
///
/// Provides the created integration.
pub struct BIntegrationCreate {
    pub ctx: Context,
    pub integration: Integration,
}

#[derive(Event)]
/// Dispatched when a guild integration is updated.
///
/// Provides the updated integration.
pub struct BIntegrationUpdate {
    pub ctx: Context,
    pub integration: Integration,
}

#[derive(Event)]
/// Dispatched when a stage instance is created.
///
/// Provides the created stage instance.
pub struct BStageInstanceCreate {
    pub ctx: Context,
    pub stage_instance: StageInstance,
}

#[derive(Event)]
/// Dispatched when a stage instance is updated.
///
/// Provides the updated stage instance.
pub struct BStageInstanceUpdate {
    pub ctx: Context,
    pub stage_instance: StageInstance,
}

#[derive(Event)]
/// Dispatched when a stage instance is deleted.
///
/// Provides the deleted stage instance.
pub struct BStageInstanceDelete {
    pub ctx: Context,
    pub stage_instance: StageInstance,
}

#[derive(Event)]
/// Dispatched when a thread is created or the current user is added to a private thread.
///
/// Provides the thread.
pub struct BThreadCreate {
    pub ctx: Context,
    pub thread: GuildChannel,
}

#[derive(Event)]
/// Dispatched when a thread is updated.
///
/// Provides the updated thread and the old thread data, provided the thread was cached prior
/// to dispatch.
pub struct BThreadUpdate {
    pub ctx: Context,
    pub old: Option<GuildChannel>,
    pub new: GuildChannel,
}

#[derive(Event)]
/// Dispatched when a thread is deleted.
///
/// Provides the partial data about the deleted thread and, if it was present in the cache
/// before its deletion, its full data.
pub struct BThreadDelete {
    pub ctx: Context,
    pub thread: PartialGuildChannel,
    pub full_thread_data: Option<GuildChannel>,
}

#[derive(Event)]
/// Dispatched when the current user gains access to a channel.
///
/// Provides the threads the current user can access, the thread members, the guild Id,
/// and the channel Ids of the parent channels being synced.
pub struct BThreadListSync {
    pub ctx: Context,
    pub thread_list_sync: ThreadListSyncEvent,
}

#[derive(Event)]
/// Dispatched when the [`ThreadMember`] for the current user is updated.
///
/// Provides the updated thread member.
pub struct BThreadMemberUpdate {
    pub ctx: Context,
    pub thread_member: ThreadMember,
}

#[derive(Event)]
/// Dispatched when anyone is added to or removed from a thread. If the current user does
/// not have the [`GatewayIntents::GUILDS]`, then this event will only be sent if the current
/// user was added to or removed from the thread.
///
/// Provides the added/removed members, the approximate member count of members in the thread,
/// the thread Id and its guild Id.
pub struct BThreadMembersUpdate {
    pub ctx: Context,
    pub thread_members_update: ThreadMembersUpdateEvent,
}

#[derive(Event)]
/// Dispatched when a scheduled event is created.
///
/// Provides data about the scheduled event.
pub struct BGuildScheduledEventCreate {
    pub ctx: Context,
    pub event: ScheduledEvent,
}

#[derive(Event)]
/// Dispatched when a scheduled event is updated.
///
/// Provides data about the scheduled event.
pub struct BGuildScheduledEventUpdate {
    pub ctx: Context,
    pub event: ScheduledEvent,
}

#[derive(Event)]
/// Dispatched when a scheduled event is deleted.
///
/// Provides data about the scheduled event.
pub struct BGuildScheduledEventDelete {
    pub ctx: Context,
    pub event: ScheduledEvent,
}

#[derive(Event)]
/// Dispatched when a guild member has subscribed to a scheduled event.
///
/// Provides data about the subscription.
pub struct BGuildScheduledEventUserAdd {
    pub ctx: Context,
    pub subscribed: GuildScheduledEventUserAddEvent,
}

#[derive(Event)]
/// Dispatched when a guild member has unsubscribed from a scheduled event.
///
/// Provides data about the cancelled subscription.
pub struct BGuildScheduledEventUserRemove {
    pub ctx: Context,
    pub unsubscribed: GuildScheduledEventUserRemoveEvent,
}

#[derive(Event)]
/// Dispatched when a user subscribes to a SKU.
///
/// Provides data about the subscription.
pub struct BEntitlementCreate {
    pub ctx: Context,
    pub entitlement: Entitlement,
}

#[derive(Event)]
/// Dispatched when a user’s entitlement has been updated, such as when a subscription is
/// renewed for the next billing period.
///
/// Provides data abut the updated subscription. If the entitlement is renewed, the
/// `[Entitlement::ends_at`] field will have changed.
pub struct BEntitlementUpdate {
    pub ctx: Context,
    pub entitlement: Entitlement,
}

#[derive(Event)]
/// Dispatched when a user’s entitlement has been deleted. This happens rarely, but can occur
/// if a subscription is refunded or otherwise deleted by Discord. Entitlements are not deleted
/// when they expire.
///
/// Provides data about the subscription. Specifically, the Entitlement::deleted field will be set.
pub struct BEntitlementDelete {
    pub ctx: Context,
    pub entitlement: Entitlement,
}

#[derive(Event)]
/// Dispatched when a user votes on a message poll.
///
/// This will be dispatched multiple times if multiple answers are selected.
pub struct BPollVoteAdd {
    pub ctx: Context,
    pub event: MessagePollVoteAddEvent,
}

#[derive(Event)]
/// Dispatched when a user removes a previous vote on a poll.
pub struct BPollVoteRemove {
    pub ctx: Context,
    pub event: MessagePollVoteRemoveEvent,
}

#[derive(Event)]
/// Dispatched when an HTTP rate limit is hit
pub struct BRateLimit {
    pub data: RatelimitInfo,
}
