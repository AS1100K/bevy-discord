// Not Accessible Publicly

//! Discord bot integration for Bevy applications.
//!
//! This module provides a plugin system for integrating a Discord bot into your Bevy application,
//! handling Discord events, and managing the bot's state. It wraps Serenity's client functionality
//! while providing a Bevy-friendly interface.
//!
//! # Features
//!
//! - Complete Discord event integration with Bevy's event system
//! - Bot configuration management (status, activity, intents)
//! - Asynchronous event handling
//!
//! # Example
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_discord::bot::{DiscordBotPlugin, DiscordBotConfig};
//! use bevy_discord::serenity::all::GatewayIntents;
//!
//! let config = DiscordBotConfig::default()
//!     .token("your-bot-token".to_string())
//!     .gateway_intents(GatewayIntents::non_privileged());
//!
//! App::new()
//!     .add_plugins(DiscordBotPlugin::new(config))
//!     .run();
//! ```
//!
//! # Note
//!
//! For HTTP interactions with Discord's API, see the [`http`](crate::http) module.

use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;
use serenity::all::*;

use crate::messages::{MessageCollectionBot, bot::*, send_events_bot};
use event_handlers::*;

use crate::DiscordSystems;
use crate::bot::handle::Handle;
use crate::channel::ChannelRes;
use crate::runtime::tokio_runtime;

pub(crate) mod event_handlers;
mod handle;

/// A plugin that integrates Discord bot functionality into a Bevy application.
///
/// # Functionality
///
/// This plugin handles:
/// - Discord client initialization and lifecycle management
/// - Event system integration between Discord and Bevy
/// - Bot presence and status management
/// - Gateway connection and communication
///
/// # Usage
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_discord::bot::{DiscordBotPlugin, DiscordBotConfig};
/// use bevy_discord::serenity::all::{GatewayIntents, ActivityData, ActivityType};
///
/// // Configure your bot
/// let config = DiscordBotConfig::default()
///     .token("your-bot-token".to_string())
///     .gateway_intents(GatewayIntents::non_privileged())
///     .activity(ActivityData::playing("with Bevy!"));
///
/// App::new()
///     .add_plugins(DiscordBotPlugin::new(config))
///     .run();
/// ```
///
/// # Features
///
/// - Automatically makes available [DiscordHttpResource](crate::res::DiscordHttpResource)
/// - Registers all Discord events as Bevy events
/// - Manages bot configuration and presence
/// - Provides asynchronous event handling
///
/// # Note
///
/// This plugin requires a valid Discord bot token and appropriate gateway intents
/// to function correctly. Make sure to configure the necessary intents based on
/// your bot's requirements.
pub struct DiscordBotPlugin(crate::config::DiscordBotConfig);

impl DiscordBotPlugin {
    /// Creates a new instance of `DiscordBotPlugin` with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `configuration` - Bot configuration including token, intents, and presence settings
    pub fn new(configuration: crate::config::DiscordBotConfig) -> Self {
        Self(configuration)
    }
}

impl Plugin for DiscordBotPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = flume::unbounded::<MessageCollectionBot>();
        let channel_res = ChannelRes { tx, rx };
        app.insert_resource(channel_res);

        #[cfg(feature = "bot_cache")]
        app.add_message::<CacheReadMessage>()
            .add_message::<ShardsReadyMessage>();

        app.insert_resource(self.0.clone())
            .add_message::<BotReadyMessage>()
            .add_message::<CommandPermissionsUpdateMessage>()
            .add_message::<AutoModerationRuleCreateMessage>()
            .add_message::<AutoModerationRuleUpdateMessage>()
            .add_message::<AutoModerationRuleDeleteMessage>()
            .add_message::<AutoModerationActionExecutionMessage>()
            .add_message::<ChannelCreateMessage>()
            .add_message::<CategoryCreateMessage>()
            .add_message::<CategoryDeleteMessage>()
            .add_message::<ChannelDeleteMessage>()
            .add_message::<ChannelPinUpdateMessage>()
            .add_message::<ChannelUpdateMessage>()
            .add_message::<GuildAuditLogEntryCreateMessage>()
            .add_message::<GuildBanAdditionMessage>()
            .add_message::<GuildBanRemovalMessage>()
            .add_message::<GuildCreateMessage>()
            .add_message::<GuildDeleteMessage>()
            .add_message::<GuildEmojisUpdateMessage>()
            .add_message::<GuildIntegrationsUpdateMessage>()
            .add_message::<GuildMemberAdditionMessage>()
            .add_message::<GuildMemberRemovalMessage>()
            .add_message::<GuildMemberUpdateMessage>()
            .add_message::<GuildMembersChunkMessage>()
            .add_message::<GuildRoleCreateMessage>()
            .add_message::<GuildRoleDeleteMessage>()
            .add_message::<GuildRoleUpdateMessage>()
            .add_message::<GuildStickersUpdateMessage>()
            .add_message::<GuildUpdateMessage>()
            .add_message::<InviteCreateMessage>()
            .add_message::<InviteDeleteMessage>()
            .add_message::<DiscordMessage>()
            .add_message::<DiscordMessageDeleteMessage>()
            .add_message::<DiscordMessageDeleteBulkMessage>()
            .add_message::<DiscordMessageUpdateMessage>()
            .add_message::<ReactionAddMessage>()
            .add_message::<ReactionRemoveMessage>()
            .add_message::<ReactionRemoveAllMessage>()
            .add_message::<ReactionRemoveEmojiMessage>()
            .add_message::<PresenceUpdateMessage>()
            .add_message::<ResumeMessage>()
            .add_message::<ShardStageUpdateMessage>()
            .add_message::<TypingStartMessage>()
            .add_message::<UserUpdateMessage>()
            .add_message::<VoiceServerUpdateMessage>()
            .add_message::<VoiceStateUpdateMessage>()
            .add_message::<VoiceChannelStatusUpdateMessage>()
            .add_message::<WebhookUpdateMessage>()
            .add_message::<InteractionCreateMessage>()
            .add_message::<IntegrationCreateMessage>()
            .add_message::<IntegrationUpdateMessage>()
            .add_message::<StageInstanceCreateMessage>()
            .add_message::<StageInstanceUpdateMessage>()
            .add_message::<StageInstanceDeleteMessage>()
            .add_message::<ThreadCreateMessage>()
            .add_message::<ThreadUpdateMessage>()
            .add_message::<ThreadDeleteMessage>()
            .add_message::<ThreadListSyncMessage>()
            .add_message::<ThreadMemberUpdateMessage>()
            .add_message::<ThreadMembersUpdateMessage>()
            .add_message::<GuildScheduledEventCreateMessage>()
            .add_message::<GuildScheduledEventUpdateMessage>()
            .add_message::<GuildScheduledEventDeleteMessage>()
            .add_message::<GuildScheduledEventUserAddMessage>()
            .add_message::<GuildScheduledEventUserRemoveMessage>()
            .add_message::<EntitlementCreateMessage>()
            .add_message::<EntitlementUpdateMessage>()
            .add_message::<EntitlementDeleteMessage>()
            .add_message::<PollVoteAddMessage>()
            .add_message::<PollVoteRemoveMessage>()
            .add_message::<RateLimitMessage>()
            .add_systems(Startup, setup_bot.in_set(DiscordSystems))
            .add_systems(
                Update,
                (handle_b_ready_event, send_events_bot)
                    .chain()
                    .in_set(DiscordSystems),
            );
    }
}

fn setup_bot(
    discord_bot_config: Res<crate::config::DiscordBotConfig>,
    channel_res: Res<ChannelRes<MessageCollectionBot>>,
) {
    let tx = channel_res.tx.clone();

    let mut client_builder = Client::builder(
        &discord_bot_config.token,
        discord_bot_config.gateway_intents,
    )
    .event_handler(Handle { tx });

    let discord_bot_res_clone = discord_bot_config.clone();

    if let Some(status) = discord_bot_res_clone.status {
        client_builder = client_builder.status(status);
    }

    if let Some(activity) = discord_bot_res_clone.activity {
        client_builder = client_builder.activity(activity);
    }

    let discord_bot_config_clone = discord_bot_config.clone();

    tokio_runtime().spawn(async move {
        let mut client = client_builder
            .await
            .expect("Unable to build discord Client");

        if discord_bot_config_clone.shards == 0 {
            client
                .start()
                .await
                .expect("Unable to run the discord Client");
        } else {
            client
                .start_shards(discord_bot_config_clone.shards)
                .await
                .expect("Unable to run the discord Client with multiple shards.")
        }
    });
}
