//! Contains all the Bevy messages that can be emitted by the Discord integration.
//!
//! This module is split into two main feature-gated submodules:
//!
//! - [`bot`] - Events related to Discord bot functionality (requires `bot` feature)
//! - [`rich_presence`] - Events related to Discord Rich Presence integration (requires `rich_presence` feature)

use crate::common::create_message_collection_and_handler;

#[cfg(feature = "bot")]
#[cfg_attr(docsrs, doc(cfg(feature = "bot")))]
pub mod bot {
    //! This module contains all the bevy [Message] that are send by `bot` feature

    use bevy_ecs::prelude::Message;
    use serenity::all::*;
    use serenity::model::channel::Message as SMessage;
    use std::collections::HashMap;

    /// Dispatched upon startup.
    ///
    /// Provides data about the bot and the guilds it’s in.
    ///
    /// Once, dispatched it automatically inserts [DiscordHttpResource](crate::res::DiscordHttpResource)
    /// to the bevy app.
    #[derive(Message, Debug, Clone)]
    pub struct BotReadyMessage {
        pub ctx: Context,
        pub data_about_bot: Ready,
    }

    /// Dispatched when the permissions of an application command was updated.
    ///
    /// Provides said permission’s data.
    #[derive(Message, Debug, Clone)]
    pub struct CommandPermissionsUpdateMessage {
        pub ctx: Context,
        pub permission: CommandPermissions,
    }

    /// Dispatched when an auto moderation rule was created.
    ///
    /// Provides said rule’s data.
    #[derive(Message, Debug, Clone)]
    pub struct AutoModerationRuleCreateMessage {
        pub ctx: Context,
        pub rule: Rule,
    }

    /// Dispatched when an auto moderation rule was updated.
    ///
    /// Provides said rule’s data.
    #[derive(Message, Debug, Clone)]
    pub struct AutoModerationRuleUpdateMessage {
        pub ctx: Context,
        pub rule: Rule,
    }

    /// Dispatched when an auto moderation rule was deleted.
    ///
    /// Provides said rule’s data.
    #[derive(Message, Debug, Clone)]
    pub struct AutoModerationRuleDeleteMessage {
        pub ctx: Context,
        pub rule: Rule,
    }

    /// Dispatched when an auto moderation rule was triggered and an action was executed.
    ///
    /// Provides said action execution’s data.
    #[derive(Message, Debug, Clone)]
    pub struct AutoModerationActionExecutionMessage {
        pub ctx: Context,
        pub execution: ActionExecution,
    }

    /// Dispatched when the cache has received and inserted all data from guilds.
    ///
    /// This process happens upon starting your bot and should be fairly quick. However, cache actions
    /// performed prior this event may fail as the data could be not inserted yet.
    ///
    /// Provides the cached guilds’ ids.
    #[cfg(feature = "bot_cache")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bot_cache")))]
    #[derive(Message, Debug, Clone)]
    pub struct CacheReadMessage {
        pub ctx: Context,
        pub guilds: Vec<GuildId>,
    }

    /// Dispatched when every shard has received a Ready event
    #[cfg(feature = "bot_cache")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bot_cache")))]
    #[derive(Message, Debug, Clone)]
    pub struct ShardsReadyMessage {
        pub ctx: Context,
        pub total_shards: u32,
    }

    /// Dispatched when a channel is created.
    ///
    /// Provides said channel’s data.
    #[derive(Message, Debug, Clone)]
    pub struct ChannelCreateMessage {
        pub ctx: Context,
        pub channel: GuildChannel,
    }

    /// Dispatched when a category is created.
    ///
    /// Provides said category’s data.
    #[derive(Message, Debug, Clone)]
    pub struct CategoryCreateMessage {
        pub ctx: Context,
        pub category: GuildChannel,
    }

    /// Dispatched when a category is deleted.
    ///
    /// Provides said category’s data.
    #[derive(Message, Debug, Clone)]
    pub struct CategoryDeleteMessage {
        pub ctx: Context,
        pub category: GuildChannel,
    }

    /// Dispatched when a channel is deleted.
    ///
    /// Provides said channel’s data.
    #[derive(Message, Debug, Clone)]
    pub struct ChannelDeleteMessage {
        pub ctx: Context,
        pub channel: GuildChannel,
        pub messages: Option<Vec<SMessage>>,
    }

    /// Dispatched when a pin is added, deleted.
    ///
    /// Provides said pin’s data.
    #[derive(Message, Debug, Clone)]
    pub struct ChannelPinUpdateMessage {
        pub ctx: Context,
        pub pin: ChannelPinsUpdateEvent,
    }

    /// Dispatched when a channel is updated.
    ///
    /// The old channel data is only provided when the `bot_cache` feature is enabled.
    #[derive(Message, Debug, Clone)]
    pub struct ChannelUpdateMessage {
        pub ctx: Context,
        pub old: Option<GuildChannel>,
        pub new: GuildChannel,
    }

    /// Dispatched when a new audit log entry is created.
    ///
    /// Provides said entry’s data and the id of the guild where it was created.
    #[derive(Message, Debug, Clone)]
    pub struct GuildAuditLogEntryCreateMessage {
        pub ctx: Context,
        pub entry: AuditLogEntry,
        pub guild_id: GuildId,
    }

    /// Dispatched when a user is banned from a guild.
    ///
    /// Provides the guild’s id and the banned user’s data.
    #[derive(Message, Debug, Clone)]
    pub struct GuildBanAdditionMessage {
        pub ctx: Context,
        pub guild_id: GuildId,
        pub banned_user: User,
    }

    /// Dispatched when a user’s ban is lifted from a guild.
    ///
    /// Provides the guild’s id and the lifted user’s data.
    #[derive(Message, Debug, Clone)]
    pub struct GuildBanRemovalMessage {
        pub ctx: Context,
        pub guild_id: GuildId,
        pub unbanned_user: User,
    }

    /// Dispatched when a guild is created; or an existing guild’s data is sent to us.
    ///
    /// Provides the guild’s data and whether the guild is new (only when `bot_cache` feature is enabled).
    #[derive(Message, Debug, Clone)]
    pub struct GuildCreateMessage {
        pub ctx: Context,
        pub guild: Guild,
        pub is_new: Option<bool>,
    }

    /// Dispatched when a guild is deleted.
    ///
    /// Provides the partial data of the guild sent by discord, and the full data from the cache,
    /// if `bot_cache` feature is enabled and the data is available.
    ///
    /// The [`UnavailableGuild::unavailable`] flag in the partial data determines the status of the guild. If the flag
    /// is false, the bot was removed from the guild, either by being kicked or banned. If the
    /// flag is true, the guild went offline.
    #[derive(Message, Debug, Clone)]
    pub struct GuildDeleteMessage {
        pub ctx: Context,
        pub incomplete: UnavailableGuild,
        pub full: Option<Guild>,
    }

    /// Dispatched when the emojis are updated.
    ///
    /// Provides the guild’s id and the new state of the emojis in the guild.
    #[derive(Message, Debug, Clone)]
    pub struct GuildEmojisUpdateMessage {
        pub ctx: Context,
        pub guild_id: GuildId,
        pub current_state: HashMap<EmojiId, Emoji>,
    }

    /// Dispatched when a guild’s integration is added, updated or removed.
    ///
    /// Provides the guild’s id.
    #[derive(Message, Debug, Clone)]
    pub struct GuildIntegrationsUpdateMessage {
        pub ctx: Context,
        pub guild_id: GuildId,
    }

    /// Dispatched when a user joins a guild.
    ///
    /// Provides the guild’s id and the user’s member data.
    ///
    /// Note: This event will not trigger unless the “guild members” privileged intent is enabled
    /// on the bot application page.
    #[derive(Message, Debug, Clone)]
    pub struct GuildMemberAdditionMessage {
        pub ctx: Context,
        pub new_member: Member,
    }

    /// Dispatched when a user’s membership ends by leaving, getting kicked, or being banned.
    ///
    /// Provides the guild’s id, the user’s data, and the user’s member data if cache feature is
    /// enabled and the data is available.
    ///
    /// Note: This event will not trigger unless the “guild members” privileged intent is enabled
    /// on the bot application page.
    #[derive(Message, Debug, Clone)]
    pub struct GuildMemberRemovalMessage {
        pub ctx: Context,
        pub guild_id: GuildId,
        pub user: User,
        pub member_data_if_available: Option<Member>,
    }

    /// Dispatched when a member is updated (e.g their nickname is updated).
    ///
    /// Provides the member’s old and new data (if cache feature is enabled and data is available)
    /// and the new raw data about updated fields.
    ///
    /// Note: This event will not trigger unless the “guild members” privileged intent is enabled
    /// on the bot application page.
    #[derive(Message, Debug, Clone)]
    pub struct GuildMemberUpdateMessage {
        pub ctx: Context,
        pub old_if_available: Option<Member>,
        pub new: Option<Member>,
        pub event: GuildMemberUpdateEvent,
    }

    /// Dispatched when the data for offline members was requested.
    ///
    /// Provides the guild’s id and the data.
    #[derive(Message, Debug, Clone)]
    pub struct GuildMembersChunkMessage {
        pub ctx: Context,
        pub chunk: GuildMembersChunkEvent,
    }

    /// Dispatched when a role is created.
    ///
    /// Provides the guild’s id and the new role’s data.
    #[derive(Message, Debug, Clone)]
    pub struct GuildRoleCreateMessage {
        pub ctx: Context,
        pub new: Role,
    }

    /// Dispatched when a role is deleted.
    /// Provides the guild’s id, the role’s id and its data (if `bot_cache` feature is enabled
    /// and the data is available).
    #[derive(Message, Debug, Clone)]
    pub struct GuildRoleDeleteMessage {
        pub ctx: Context,
        pub guild_id: GuildId,
        pub removed_role_id: RoleId,
        pub removed_role_data_if_available: Option<Role>,
    }

    /// Dispatched when a role is updated.
    ///
    /// Provides the guild’s id, the role’s old (if `bot_cache` feature is enabled and the data
    /// is available) and new data.
    #[derive(Message, Debug, Clone)]
    pub struct GuildRoleUpdateMessage {
        pub ctx: Context,
        pub old_data_if_available: Option<Role>,
        pub new: Role,
    }

    /// Dispatched when the stickers are updated.
    ///
    /// Provides the guild’s id and the new state of the stickers in the guild.
    #[derive(Message, Debug, Clone)]
    pub struct GuildStickersUpdateMessage {
        pub ctx: Context,
        pub guild_id: GuildId,
        pub current_state: HashMap<StickerId, Sticker>,
    }

    /// Dispatched when the guild is updated.
    ///
    /// Provides the guild’s old data (if `bot_cache` feature is enabled and the data is
    /// available) and the new data.
    #[derive(Message, Debug, Clone)]
    pub struct GuildUpdateMessage {
        pub ctx: Context,
        pub old_data_if_available: Option<Guild>,
        pub new_data: PartialGuild,
    }

    /// Dispatched when a invite is created.
    ///
    /// Provides data about the invite.
    #[derive(Message, Debug, Clone)]
    pub struct InviteCreateMessage {
        pub ctx: Context,
        pub data: InviteCreateEvent,
    }

    /// Dispatched when a invite is deleted.
    ///
    /// Provides data about the invite.
    #[derive(Message, Debug, Clone)]
    pub struct InviteDeleteMessage {
        pub ctx: Context,
        pub data: InviteDeleteEvent,
    }

    /// Dispatched when a message is created.
    ///
    /// Provides the message’s data.
    // TODO: Is the name good??
    #[derive(Message, Debug, Clone)]
    pub struct DiscordMessage {
        pub ctx: Context,
        pub new_message: SMessage,
    }

    /// Dispatched when a message is deleted.
    ///
    /// Provides the guild’s id, the channel’s id and the message’s id.
    // TODO: Is the name good??
    #[derive(Message, Debug, Clone)]
    pub struct DiscordMessageDeleteMessage {
        pub ctx: Context,
        pub channel_id: ChannelId,
        pub deleted_message_id: MessageId,
        pub guild_id: Option<GuildId>,
    }

    /// Dispatched when multiple messages were deleted at once.
    ///
    /// Provides the guild’s id, channel’s id and the deleted messages’ ids.
    // TODO: Is the name good??
    #[derive(Message, Debug, Clone)]
    pub struct DiscordMessageDeleteBulkMessage {
        pub ctx: Context,
        pub channel_id: ChannelId,
        pub multiple_deleted_messages_ids: Vec<MessageId>,
        pub guild_id: Option<GuildId>,
    }

    /// Dispatched when a message is updated.
    ///
    /// Provides the message update data, as well as the actual old and new message if `bot_cache`
    /// feature is enabled and the data is available.
    // TODO: Is the name good??
    #[derive(Message, Debug, Clone)]
    pub struct DiscordMessageUpdateMessage {
        pub ctx: Context,
        pub old_if_available: Option<SMessage>,
        pub new: Option<SMessage>,
        pub event: MessageUpdateEvent,
    }

    /// Dispatched when a new reaction is attached to a message.
    ///
    /// Provides the reaction’s data.
    #[derive(Message, Debug, Clone)]
    pub struct ReactionAddMessage {
        pub ctx: Context,
        pub add_reaction: Reaction,
    }

    /// Dispatched when a reaction is detached from a message.
    ///
    /// Provides the reaction’s data.
    #[derive(Message, Debug, Clone)]
    pub struct ReactionRemoveMessage {
        pub ctx: Context,
        pub removed_reaction: Reaction,
    }

    /// Dispatched when all reactions of a message are detached from a message.
    ///
    /// Provides the channel’s id and the message’s id.
    #[derive(Message, Debug, Clone)]
    pub struct ReactionRemoveAllMessage {
        pub ctx: Context,
        pub channel_id: ChannelId,
        pub removed_from_message_id: MessageId,
    }

    /// Dispatched when all reactions of a message are detached from a message.
    ///
    /// Provides the channel’s id and the message’s id.
    #[derive(Message, Debug, Clone)]
    pub struct ReactionRemoveEmojiMessage {
        pub ctx: Context,
        pub removed_reactions: Reaction,
    }

    /// Dispatched when a user’s presence is updated (e.g off -> on).
    ///
    /// Provides the presence’s new data.
    ///
    /// Note: This event will not trigger unless the “guild presences” privileged intent is enabled
    /// on the bot application page.
    #[derive(Message, Debug, Clone)]
    pub struct PresenceUpdateMessage {
        pub ctx: Context,
        pub new_data: Presence,
    }

    /// Dispatched upon reconnection.
    #[derive(Message, Debug, Clone)]
    pub struct ResumeMessage {
        pub ctx: Context,
        pub event: ResumedEvent,
    }

    /// Dispatched when a shard’s connection stage is updated
    ///
    /// Provides the context of the shard and the event information about the update.
    #[derive(Message, Debug, Clone)]
    pub struct ShardStageUpdateMessage {
        pub ctx: Context,
        pub event: ShardStageUpdateEvent,
    }

    /// Dispatched when a user starts typing.
    #[derive(Message, Debug, Clone)]
    pub struct TypingStartMessage {
        pub ctx: Context,
        pub event: TypingStartEvent,
    }

    /// Dispatched when the bot’s data is updated.
    ///
    /// Provides the old (if `bot_cache` feature is enabled and the data is available) and new data.
    #[derive(Message, Debug, Clone)]
    pub struct UserUpdateMessage {
        pub ctx: Context,
        pub old_data: Option<CurrentUser>,
        pub new: CurrentUser,
    }

    /// Dispatched when a guild’s voice server was updated (or changed to another one).
    ///
    /// Provides the voice server’s data.
    #[derive(Message, Debug, Clone)]
    pub struct VoiceServerUpdateMessage {
        pub ctx: Context,
        pub event: VoiceServerUpdateEvent,
    }

    /// Dispatched when a user joins, leaves or moves to a voice channel.
    ///
    /// Provides the guild’s id (if available) and the old state (if `bot_cache` feature is enabled
    /// and [`GatewayIntents::GUILDS`] is enabled) and the new state of the guild’s voice channels.
    #[derive(Message, Debug, Clone)]
    pub struct VoiceStateUpdateMessage {
        pub ctx: Context,
        pub old: Option<VoiceState>,
        pub new: VoiceState,
    }

    /// Dispatched when a voice channel’s status is updated.
    ///
    /// Provides the status, channel’s id and the guild’s id.
    #[derive(Message, Debug, Clone)]
    pub struct VoiceChannelStatusUpdateMessage {
        pub ctx: Context,
        pub old: Option<String>,
        pub status: Option<String>,
        pub id: ChannelId,
        pub guild_id: GuildId,
    }

    /// Dispatched when a guild’s webhook is updated.
    ///
    /// Provides the guild’s id and the channel’s id the webhook belongs in.
    #[derive(Message, Debug, Clone)]
    pub struct WebhookUpdateMessage {
        pub ctx: Context,
        pub guild_id: GuildId,
        pub belongs_to_channel_id: ChannelId,
    }

    /// Dispatched when an interaction is created (e.g a slash command was used or a button was
    /// clicked).
    ///
    /// Provides the created interaction.
    #[derive(Message, Debug, Clone)]
    pub struct InteractionCreateMessage {
        pub ctx: Context,
        pub interaction: Interaction,
    }

    /// Dispatched when a guild integration is created.
    ///
    /// Provides the created integration.
    #[derive(Message, Debug, Clone)]
    pub struct IntegrationCreateMessage {
        pub ctx: Context,
        pub integration: Integration,
    }

    /// Dispatched when a guild integration is updated.
    ///
    /// Provides the updated integration.
    #[derive(Message, Debug, Clone)]
    pub struct IntegrationUpdateMessage {
        pub ctx: Context,
        pub integration: Integration,
    }

    /// Dispatched when a stage instance is created.
    ///
    /// Provides the created stage instance.
    #[derive(Message, Debug, Clone)]
    pub struct StageInstanceCreateMessage {
        pub ctx: Context,
        pub stage_instance: StageInstance,
    }

    /// Dispatched when a stage instance is updated.
    ///
    /// Provides the updated stage instance.
    #[derive(Message, Debug, Clone)]
    pub struct StageInstanceUpdateMessage {
        pub ctx: Context,
        pub stage_instance: StageInstance,
    }

    /// Dispatched when a stage instance is deleted.
    ///
    /// Provides the deleted stage instance.
    #[derive(Message, Debug, Clone)]
    pub struct StageInstanceDeleteMessage {
        pub ctx: Context,
        pub stage_instance: StageInstance,
    }

    /// Dispatched when a thread is created or the current user is added to a private thread.
    ///
    /// Provides the thread.
    #[derive(Message, Debug, Clone)]
    pub struct ThreadCreateMessage {
        pub ctx: Context,
        pub thread: GuildChannel,
    }

    /// Dispatched when a thread is updated.
    ///
    /// Provides the updated thread and the old thread data, provided the thread was cached prior
    /// to dispatch.
    #[derive(Message, Debug, Clone)]
    pub struct ThreadUpdateMessage {
        pub ctx: Context,
        pub old: Option<GuildChannel>,
        pub new: GuildChannel,
    }

    /// Dispatched when a thread is deleted.
    ///
    /// Provides the partial data about the deleted thread and, if it was present in the cache
    /// before its deletion, its full data.
    #[derive(Message, Debug, Clone)]
    pub struct ThreadDeleteMessage {
        pub ctx: Context,
        pub thread: PartialGuildChannel,
        pub full_thread_data: Option<GuildChannel>,
    }

    /// Dispatched when the current user gains access to a channel.
    ///
    /// Provides the threads the current user can access, the thread members, the guild Id,
    /// and the channel Ids of the parent channels being synced.
    #[derive(Message, Debug, Clone)]
    pub struct ThreadListSyncMessage {
        pub ctx: Context,
        pub thread_list_sync: ThreadListSyncEvent,
    }

    /// Dispatched when the [`ThreadMember`] for the current user is updated.
    ///
    /// Provides the updated thread member.
    #[derive(Message, Debug, Clone)]
    pub struct ThreadMemberUpdateMessage {
        pub ctx: Context,
        pub thread_member: ThreadMember,
    }

    /// Dispatched when anyone is added to or removed from a thread. If the current user does
    /// not have the [`GatewayIntents::GUILDS]`, then this event will only be sent if the current
    /// user was added to or removed from the thread.
    ///
    /// Provides the added/removed members, the approximate member count of members in the thread,
    /// the thread Id and its guild Id.
    #[derive(Message, Debug, Clone)]
    pub struct ThreadMembersUpdateMessage {
        pub ctx: Context,
        pub thread_members_update: ThreadMembersUpdateEvent,
    }

    /// Dispatched when a scheduled event is created.
    ///
    /// Provides data about the scheduled event.
    #[derive(Message, Debug, Clone)]
    pub struct GuildScheduledEventCreateMessage {
        pub ctx: Context,
        pub event: ScheduledEvent,
    }

    /// Dispatched when a scheduled event is updated.
    ///
    /// Provides data about the scheduled event.
    #[derive(Message, Debug, Clone)]
    pub struct GuildScheduledEventUpdateMessage {
        pub ctx: Context,
        pub event: ScheduledEvent,
    }

    /// Dispatched when a scheduled event is deleted.
    ///
    /// Provides data about the scheduled event.
    #[derive(Message, Debug, Clone)]
    pub struct GuildScheduledEventDeleteMessage {
        pub ctx: Context,
        pub event: ScheduledEvent,
    }

    /// Dispatched when a guild member has subscribed to a scheduled event.
    ///
    /// Provides data about the subscription.
    #[derive(Message, Debug, Clone)]
    pub struct GuildScheduledEventUserAddMessage {
        pub ctx: Context,
        pub subscribed: GuildScheduledEventUserAddEvent,
    }

    /// Dispatched when a guild member has unsubscribed from a scheduled event.
    ///
    /// Provides data about the cancelled subscription.
    #[derive(Message, Debug, Clone)]
    pub struct GuildScheduledEventUserRemoveMessage {
        pub ctx: Context,
        pub unsubscribed: GuildScheduledEventUserRemoveEvent,
    }

    /// Dispatched when a user subscribes to a SKU.
    ///
    /// Provides data about the subscription.
    #[derive(Message, Debug, Clone)]
    pub struct EntitlementCreateMessage {
        pub ctx: Context,
        pub entitlement: Entitlement,
    }

    /// Dispatched when a user’s entitlement has been updated, such as when a subscription is
    /// renewed for the next billing period.
    ///
    /// Provides data abut the updated subscription. If the entitlement is renewed, the
    /// `[Entitlement::ends_at`] field will have changed.
    #[derive(Message, Debug, Clone)]
    pub struct EntitlementUpdateMessage {
        pub ctx: Context,
        pub entitlement: Entitlement,
    }

    /// Dispatched when a user’s entitlement has been deleted. This happens rarely, but can occur
    /// if a subscription is refunded or otherwise deleted by Discord. Entitlements are not deleted
    /// when they expire.
    ///
    /// Provides data about the subscription. Specifically, the Entitlement::deleted field will be set.
    #[derive(Message, Debug, Clone)]
    pub struct EntitlementDeleteMessage {
        pub ctx: Context,
        pub entitlement: Entitlement,
    }

    /// Dispatched when a user votes on a message poll.
    ///
    /// This will be dispatched multiple times if multiple answers are selected.
    #[derive(Message, Debug, Clone)]
    pub struct PollVoteAddMessage {
        pub ctx: Context,
        pub event: MessagePollVoteAddEvent,
    }

    /// Dispatched when a user removes a previous vote on a poll.
    #[derive(Message, Debug, Clone)]
    pub struct PollVoteRemoveMessage {
        pub ctx: Context,
        pub event: MessagePollVoteRemoveEvent,
    }

    /// Dispatched when an HTTP rate limit is hit
    #[derive(Message, Debug, Clone)]
    pub struct RateLimitMessage {
        pub data: RatelimitInfo,
    }
}

#[cfg(feature = "rich_presence")]
#[cfg_attr(docsrs, doc(cfg(feature = "rich_presence")))]
pub mod rich_presence {
    //! This module contains all the bevy [Message] thrown by `rich_presence` feature

    use bevy_ecs::prelude::Message;
    use discord_sdk::activity::ActivityInvite;
    use discord_sdk::overlay::Visibility;
    use discord_sdk::relations::Relationship;
    use discord_sdk::user::User;
    use discord_sdk::{Error, Event};
    use std::sync::Arc;

    /// Fires when we’ve done something naughty and Discord is telling us to stop.
    ///
    /// The [Event] will always be of type [Event::Error]
    #[derive(Message, Debug)]
    pub struct ErrorMessage(pub Event);

    /// Sent by Discord upon receipt of our Handshake message, the user is the current user logged
    /// in to the Discord we connected to.
    #[derive(Message, Debug, Clone)]
    pub struct RpReadyMessage {
        /// The protocol version, we only support v1, which is fine since that is (currently) the only version
        pub version: u32,
        // `DiscordConfig` is private in `ConnectEvent`
        // pub config: DiscordConfig
        /// The user that is logged into the Discord application we connected to
        pub user: User,
    }

    /// Fired when the connection has been interrupted between us and Discord, this is a synthesized
    /// event as there are can be numerous reasons on the client side for this to happen, in
    /// addition to Discord itself being closed, etc.
    #[derive(Message, Debug)]
    pub struct DisconnectedMessage {
        pub reason: Error,
    }

    /// Fired when any details on the current logged in user are changed.
    #[derive(Message, Debug, Clone)]
    pub struct CurrentUserUpdateMessage {
        /// The user that is logged into the Discord application we connected to
        pub user: User,
    }

    /// Sent by Discord when the local user has requested to join a game, and the remote user has accepted their request.
    #[derive(Message, Debug, Clone)]
    pub struct ActivityJoinMessage {
        pub secret: String,
    }

    /// Sent by Discord when the local user has chosen to spectate another user’s game session.
    #[derive(Message, Debug, Clone)]
    pub struct ActivitySpectateMessage {
        pub secret: String,
    }

    /// Fires when a user asks to join the current user’s game.
    #[derive(Message, Debug, Clone)]
    pub struct ActivityJoinRequestMessage {
        /// Payload for the event fired when a user “Asks to Join” the current user’s game
        pub user: User,
    }

    /// Fires when the current user is invited by another user to their game.
    #[derive(Message, Debug, Clone)]
    pub struct ActivityInviteMessage(pub Arc<ActivityInvite>);

    /// Event fired when the overlay state changes.
    #[derive(Message, Debug, Clone)]
    pub struct OverlayUpdateMessage {
        /// Whether the user has the overlay enabled or disabled. If the overlay is disabled, all
        /// the functionality of the SDK will still work. The calls will instead focus the Discord
        /// client and show the modal there instead of in application.
        pub enabled: bool,
        /// Whether the overlay is visible or not.
        pub visible: Visibility,
    }

    /// Event fired when a relationship with another user changes.
    #[derive(Message, Debug, Clone)]
    pub struct RelationshipUpdateMessage(pub Arc<Relationship>);
}

#[cfg(feature = "bot")]
use bot::*;
#[cfg(feature = "rich_presence")]
use rich_presence::*;

#[cfg(feature = "bot")]
create_message_collection_and_handler!(
    MessageCollectionBot,
    bot,
    CommandPermissionsUpdateMessage,
    AutoModerationRuleCreateMessage,
    AutoModerationRuleUpdateMessage,
    AutoModerationRuleDeleteMessage,
    AutoModerationActionExecutionMessage,
    #[cfg(all(feature = "bot_cache", feature = "bot_cache"))]
    CacheReadMessage,
    #[cfg(all(feature = "bot_cache", feature = "bot_cache"))]
    ShardsReadyMessage,
    ChannelCreateMessage,
    CategoryCreateMessage,
    CategoryDeleteMessage,
    ChannelDeleteMessage,
    ChannelPinUpdateMessage,
    ChannelUpdateMessage,
    GuildAuditLogEntryCreateMessage,
    GuildBanAdditionMessage,
    GuildBanRemovalMessage,
    GuildCreateMessage,
    GuildDeleteMessage,
    GuildEmojisUpdateMessage,
    GuildIntegrationsUpdateMessage,
    GuildMemberAdditionMessage,
    GuildMemberRemovalMessage,
    GuildMemberUpdateMessage,
    GuildMembersChunkMessage,
    GuildRoleCreateMessage,
    GuildRoleDeleteMessage,
    GuildRoleUpdateMessage,
    GuildStickersUpdateMessage,
    GuildUpdateMessage,
    InviteCreateMessage,
    InviteDeleteMessage,
    DiscordMessage,
    DiscordMessageDeleteMessage,
    DiscordMessageDeleteBulkMessage,
    DiscordMessageUpdateMessage,
    ReactionAddMessage,
    ReactionRemoveMessage,
    ReactionRemoveAllMessage,
    ReactionRemoveEmojiMessage,
    PresenceUpdateMessage,
    BotReadyMessage,
    ResumeMessage,
    ShardStageUpdateMessage,
    TypingStartMessage,
    UserUpdateMessage,
    VoiceServerUpdateMessage,
    VoiceStateUpdateMessage,
    VoiceChannelStatusUpdateMessage,
    WebhookUpdateMessage,
    InteractionCreateMessage,
    IntegrationCreateMessage,
    IntegrationUpdateMessage,
    StageInstanceCreateMessage,
    StageInstanceUpdateMessage,
    StageInstanceDeleteMessage,
    ThreadCreateMessage,
    ThreadUpdateMessage,
    ThreadDeleteMessage,
    ThreadListSyncMessage,
    ThreadMemberUpdateMessage,
    ThreadMembersUpdateMessage,
    GuildScheduledEventCreateMessage,
    GuildScheduledEventUpdateMessage,
    GuildScheduledEventDeleteMessage,
    GuildScheduledEventUserAddMessage,
    GuildScheduledEventUserRemoveMessage,
    EntitlementCreateMessage,
    EntitlementUpdateMessage,
    EntitlementDeleteMessage,
    PollVoteAddMessage,
    PollVoteRemoveMessage,
    RateLimitMessage,
);

#[cfg(feature = "rich_presence")]
create_message_collection_and_handler!(
    MessageCollectionRichPresence,
    rich_presence,
    ErrorMessage,
    RpReadyMessage,
    DisconnectedMessage,
    CurrentUserUpdateMessage,
    ActivityJoinMessage,
    ActivitySpectateMessage,
    ActivityJoinRequestMessage,
    ActivityInviteMessage,
    OverlayUpdateMessage,
    RelationshipUpdateMessage
);
