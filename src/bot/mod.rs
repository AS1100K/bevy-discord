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
// send_events function is created with macro expansion, so don't worry when IDE tries hard to find it
use common::{send_events, BEventCollection};
use flume::Receiver;
use serenity::all::*;

use event_handlers::*;
use events::*;

use crate::bot::handle::Handle;
use crate::common::{initialize_field_with_doc, override_field_with_doc};
use crate::runtime::tokio_runtime;
use crate::DiscordSet;

mod common;
mod event_handlers;
pub mod events;
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
/// - Automatically makes available [DiscordHttpResource](crate::http::DiscordHttpResource)
/// - Registers all Discord events as Bevy events
/// - Manages bot configuration and presence
/// - Provides asynchronous event handling
///
/// # Note
///
/// This plugin requires a valid Discord bot token and appropriate gateway intents
/// to function correctly. Make sure to configure the necessary intents based on
/// your bot's requirements.
pub struct DiscordBotPlugin(DiscordBotConfig);

impl DiscordBotPlugin {
    /// Creates a new instance of `DiscordBotPlugin` with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `configuration` - Bot configuration including token, intents, and presence settings
    pub fn new(configuration: DiscordBotConfig) -> Self {
        Self(configuration)
    }
}

impl Plugin for DiscordBotPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "bot_cache")]
        app.add_event::<BCacheRead>().add_event::<BShardsReady>();

        app.insert_resource(self.0.clone())
            .add_event::<BReadyEvent>()
            .add_event::<BCommandPermissionsUpdate>()
            .add_event::<BAutoModerationRuleCreate>()
            .add_event::<BAutoModerationRuleUpdate>()
            .add_event::<BAutoModerationRuleDelete>()
            .add_event::<BAutoModerationActionExecution>()
            .add_event::<BChannelCreate>()
            .add_event::<BCategoryCreate>()
            .add_event::<BCategoryDelete>()
            .add_event::<BChannelDelete>()
            .add_event::<BChannelPinUpdate>()
            .add_event::<BChannelUpdate>()
            .add_event::<BGuildAuditLogEntryCreate>()
            .add_event::<BGuildBanAddition>()
            .add_event::<BGuildBanRemoval>()
            .add_event::<BGuildCreate>()
            .add_event::<BGuildDelete>()
            .add_event::<BGuildEmojisUpdate>()
            .add_event::<BGuildIntegrationsUpdate>()
            .add_event::<BGuildMemberAddition>()
            .add_event::<BGuildMemberRemoval>()
            .add_event::<BGuildMemberUpdate>()
            .add_event::<BGuildMembersChunk>()
            .add_event::<BGuildRoleCreate>()
            .add_event::<BGuildRoleDelete>()
            .add_event::<BGuildRoleUpdate>()
            .add_event::<BGuildStickersUpdate>()
            .add_event::<BGuildUpdate>()
            .add_event::<BInviteCreate>()
            .add_event::<BInviteDelete>()
            .add_event::<BMessage>()
            .add_event::<BMessageDelete>()
            .add_event::<BMessageDeleteBulk>()
            .add_event::<BMessageUpdate>()
            .add_event::<BReactionAdd>()
            .add_event::<BReactionRemove>()
            .add_event::<BReactionRemoveAll>()
            .add_event::<BReactionRemoveEmoji>()
            .add_event::<BPresenceUpdate>()
            .add_event::<BResume>()
            .add_event::<BShardStageUpdate>()
            .add_event::<BTypingStart>()
            .add_event::<BUserUpdate>()
            .add_event::<BVoiceServerUpdate>()
            .add_event::<BVoiceStateUpdate>()
            .add_event::<BVoiceChannelStatusUpdate>()
            .add_event::<BWebhookUpdate>()
            .add_event::<BInteractionCreate>()
            .add_event::<BIntegrationCreate>()
            .add_event::<BIntegrationUpdate>()
            .add_event::<BStageInstanceCreate>()
            .add_event::<BStageInstanceUpdate>()
            .add_event::<BStageInstanceDelete>()
            .add_event::<BThreadCreate>()
            .add_event::<BThreadUpdate>()
            .add_event::<BThreadDelete>()
            .add_event::<BThreadListSync>()
            .add_event::<BThreadMemberUpdate>()
            .add_event::<BThreadMembersUpdate>()
            .add_event::<BGuildScheduledEventCreate>()
            .add_event::<BGuildScheduledEventUpdate>()
            .add_event::<BGuildScheduledEventDelete>()
            .add_event::<BGuildScheduledEventUserAdd>()
            .add_event::<BGuildScheduledEventUserRemove>()
            .add_event::<BEntitlementCreate>()
            .add_event::<BEntitlementUpdate>()
            .add_event::<BEntitlementDelete>()
            .add_event::<BPollVoteAdd>()
            .add_event::<BPollVoteRemove>()
            .add_event::<BRateLimit>()
            .add_systems(Startup, setup_bot.in_set(DiscordSet))
            .add_systems(
                Update,
                (send_events, handle_b_ready_event)
                    .chain()
                    .in_set(DiscordSet),
            );
    }
}

/// Configuration settings for the Discord bot.
///
/// This struct allows you to configure various aspects of the bot including:
/// - Bot token
/// - Gateway intents
/// - Online status
/// - Activity status
#[derive(Default, Resource, Clone)]
pub struct DiscordBotConfig {
    token: String,
    gateway_intents: GatewayIntents,
    status: Option<OnlineStatus>,
    activity: Option<ActivityData>,
    shards: u32,
}

impl DiscordBotConfig {
    initialize_field_with_doc!(token, String, "Sets the bot token.");
    initialize_field_with_doc!(
        gateway_intents,
        GatewayIntents,
        "Sets the bot [`GatewayIntents`]."
    );
    override_field_with_doc!(status, OnlineStatus, "Sets the initial status.");
    override_field_with_doc!(activity, ActivityData, "Sets the initial activity.");
    initialize_field_with_doc!(shards, u32, "The total number of shards to use.");
}

/// A global resource for the Discord bot.
///
/// This resource maintains the bot's internal state and event communication channel.
#[derive(Resource)]
pub struct DiscordBotRes {
    pub(crate) recv: Receiver<BEventCollection>,
}

fn setup_bot(mut commands: Commands, discord_bot_config: Res<DiscordBotConfig>) {
    let (tx, rx) = flume::unbounded();

    commands.insert_resource(DiscordBotRes { recv: rx });

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
