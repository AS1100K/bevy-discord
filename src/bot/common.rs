use super::events::*;
use crate::create_event_collection_and_handler;
use bevy_ecs::prelude::*;

create_event_collection_and_handler!(
    BCommandPermissionsUpdate,
    BAutoModerationRuleCreate,
    BAutoModerationRuleUpdate,
    BAutoModerationRuleDelete,
    BAutoModerationActionExecution,
    #[cfg(feature = "bot_cache")]
    BCacheRead,
    #[cfg(feature = "bot_cache")]
    BShardsReady,
    BChannelCreate,
    BCategoryCreate,
    BCategoryDelete,
    BChannelDelete,
    BChannelPinUpdate,
    BChannelUpdate,
    BGuildAuditLogEntryCreate,
    BGuildBanAddition,
    BGuildBanRemoval,
    BGuildCreate,
    BGuildDelete,
    BGuildEmojisUpdate,
    BGuildIntegrationsUpdate,
    BGuildMemberAddition,
    BGuildMemberRemoval,
    BGuildMemberUpdate,
    BGuildMembersChunk,
    BGuildRoleCreate,
    BGuildRoleDelete,
    BGuildRoleUpdate,
    BGuildStickersUpdate,
    BGuildUpdate,
    BInviteCreate,
    BInviteDelete,
    BMessage,
    BMessageDelete,
    BMessageDeleteBulk,
    BMessageUpdate,
    BReactionAdd,
    BReactionRemove,
    BReactionRemoveAll,
    BReactionRemoveEmoji,
    BPresenceUpdate,
    BReadyEvent,
    BResume,
    BShardStageUpdate,
    BTypingStart,
    BUserUpdate,
    BVoiceServerUpdate,
    BVoiceStateUpdate,
    BVoiceChannelStatusUpdate,
    BWebhookUpdate,
    BInteractionCreate,
    BIntegrationCreate,
    BIntegrationUpdate,
    BStageInstanceCreate,
    BStageInstanceUpdate,
    BStageInstanceDelete,
    BThreadCreate,
    BThreadUpdate,
    BThreadDelete,
    BThreadListSync,
    BThreadMemberUpdate,
    BThreadMembersUpdate,
    BGuildScheduledEventCreate,
    BGuildScheduledEventUpdate,
    BGuildScheduledEventDelete,
    BGuildScheduledEventUserAdd,
    BGuildScheduledEventUserRemove,
    BEntitlementCreate,
    BEntitlementUpdate,
    BEntitlementDelete,
    BPollVoteAdd,
    BPollVoteRemove,
    BRateLimit
);
