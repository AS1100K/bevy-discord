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

use crate::events::{EventCollectionBot, bot::*, send_events_bot};
use event_handlers::*;

use crate::DiscordSet;
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
        let (tx, rx) = flume::unbounded::<EventCollectionBot>();
        let channel_res = ChannelRes { tx, rx };
        app.insert_resource(channel_res);

        #[cfg(feature = "bot_cache")]
        app.add_event::<BCacheRead>().add_event::<BShardsReady>();

        app.insert_resource(self.0.clone())
            .add_message::<BReadyEvent>()
            .add_message::<BCommandPermissionsUpdate>()
            .add_message::<BAutoModerationRuleCreate>()
            .add_message::<BAutoModerationRuleUpdate>()
            .add_message::<BAutoModerationRuleDelete>()
            .add_message::<BAutoModerationActionExecution>()
            .add_message::<BChannelCreate>()
            .add_message::<BCategoryCreate>()
            .add_message::<BCategoryDelete>()
            .add_message::<BChannelDelete>()
            .add_message::<BChannelPinUpdate>()
            .add_message::<BChannelUpdate>()
            .add_message::<BGuildAuditLogEntryCreate>()
            .add_message::<BGuildBanAddition>()
            .add_message::<BGuildBanRemoval>()
            .add_message::<BGuildCreate>()
            .add_message::<BGuildDelete>()
            .add_message::<BGuildEmojisUpdate>()
            .add_message::<BGuildIntegrationsUpdate>()
            .add_message::<BGuildMemberAddition>()
            .add_message::<BGuildMemberRemoval>()
            .add_message::<BGuildMemberUpdate>()
            .add_message::<BGuildMembersChunk>()
            .add_message::<BGuildRoleCreate>()
            .add_message::<BGuildRoleDelete>()
            .add_message::<BGuildRoleUpdate>()
            .add_message::<BGuildStickersUpdate>()
            .add_message::<BGuildUpdate>()
            .add_message::<BInviteCreate>()
            .add_message::<BInviteDelete>()
            .add_message::<BMessage>()
            .add_message::<BMessageDelete>()
            .add_message::<BMessageDeleteBulk>()
            .add_message::<BMessageUpdate>()
            .add_message::<BReactionAdd>()
            .add_message::<BReactionRemove>()
            .add_message::<BReactionRemoveAll>()
            .add_message::<BReactionRemoveEmoji>()
            .add_message::<BPresenceUpdate>()
            .add_message::<BResume>()
            .add_message::<BShardStageUpdate>()
            .add_message::<BTypingStart>()
            .add_message::<BUserUpdate>()
            .add_message::<BVoiceServerUpdate>()
            .add_message::<BVoiceStateUpdate>()
            .add_message::<BVoiceChannelStatusUpdate>()
            .add_message::<BWebhookUpdate>()
            .add_message::<BInteractionCreate>()
            .add_message::<BIntegrationCreate>()
            .add_message::<BIntegrationUpdate>()
            .add_message::<BStageInstanceCreate>()
            .add_message::<BStageInstanceUpdate>()
            .add_message::<BStageInstanceDelete>()
            .add_message::<BThreadCreate>()
            .add_message::<BThreadUpdate>()
            .add_message::<BThreadDelete>()
            .add_message::<BThreadListSync>()
            .add_message::<BThreadMemberUpdate>()
            .add_message::<BThreadMembersUpdate>()
            .add_message::<BGuildScheduledEventCreate>()
            .add_message::<BGuildScheduledEventUpdate>()
            .add_message::<BGuildScheduledEventDelete>()
            .add_message::<BGuildScheduledEventUserAdd>()
            .add_message::<BGuildScheduledEventUserRemove>()
            .add_message::<BEntitlementCreate>()
            .add_message::<BEntitlementUpdate>()
            .add_message::<BEntitlementDelete>()
            .add_message::<BPollVoteAdd>()
            .add_message::<BPollVoteRemove>()
            .add_message::<BRateLimit>()
            .add_systems(Startup, setup_bot.in_set(DiscordSet))
            .add_systems(
                Update,
                (handle_b_ready_event, send_events_bot)
                    .chain()
                    .in_set(DiscordSet),
            );
    }
}

fn setup_bot(
    discord_bot_config: Res<crate::config::DiscordBotConfig>,
    channel_res: Res<ChannelRes<EventCollectionBot>>,
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
