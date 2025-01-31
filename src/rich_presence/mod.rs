//! [Discord Rich Presence](https://discord.com/developers/docs/rich-presence/overview) Integration with bevy
//!
//! # NOTE before using
//! - This module depends on [discord-sdk](https://crates.io/crates/discord-sdk) crate
//! - Since, Discord Game SDK is undocumented, closed-source, publicly distributed library, it can
//!   result in introduction to breaking changes at any time in future by discord.

mod event_handlers;

use crate::events::rich_presence::*;
use crate::rich_presence::event_handlers::EventHandler;
use crate::DiscordSet;
use crate::{channel::ChannelRes, runtime::tokio_runtime};
use ::discord_sdk::AppId;
use bevy_app::{App, Plugin, Startup};
use bevy_ecs::prelude::*;
use discord_sdk::{Discord, Subscriptions};
use std::sync::Arc;

/// A global resource for the Discord bot.
///
/// This resource maintains the bot's internal state and event communication channel.
#[derive(Resource)]
pub struct DiscordRichPresenceRes {
    pub discord: Arc<Discord>,
}

#[derive(Resource, Clone)]
pub struct DiscordRichPresenceConfig {
    pub app: AppId,
    pub subscriptions: Subscriptions,
}

/// **If you want to use this plugin, then you should probably use [DiscordPluginGroup](crate::DiscordPluginGroup) instead.**
pub struct DiscordRichPresencePlugin(DiscordRichPresenceConfig);

impl DiscordRichPresencePlugin {
    pub fn new(discord_rich_presence_config: DiscordRichPresenceConfig) -> Self {
        Self(discord_rich_presence_config)
    }
}

impl Plugin for DiscordRichPresencePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone())
            .add_event::<RichPresenceError>()
            .add_event::<RichPresenceReady>()
            .add_event::<RichPresenceDisconnected>()
            .add_event::<RichPresenceCurrentUserUpdate>()
            .add_event::<RichPresenceActivityJoin>()
            .add_event::<RichPresenceActivitySpectate>()
            .add_event::<RichPresenceActivityJoinRequest>()
            .add_event::<RichPresenceActivityInvite>()
            .add_event::<RichPresenceOverlayUpdate>()
            .add_event::<RichPresenceRelationshipUpdate>()
            .add_systems(
                Startup,
                setup_rich_presence
                    .in_set(DiscordSet)
                    .run_if(resource_exists::<ChannelRes>),
            );
    }
}

fn setup_rich_presence(
    mut commands: Commands,
    discord_rich_presence_config: Res<DiscordRichPresenceConfig>,
    channel_res: Res<ChannelRes>,
) {
    let tx = channel_res.tx.clone();
    let event_handler = Box::new(EventHandler { tx });

    let discord_rich_presence_config = discord_rich_presence_config.clone();
    let discord_res = || {
        commands.insert_resource(DiscordRichPresenceRes {
            discord: Arc::new(
                Discord::new(
                    discord_rich_presence_config.app,
                    discord_rich_presence_config.subscriptions,
                    event_handler,
                )
                .expect("Failed to create a Discord Rich Presence Client"),
            ),
        });
    };

    tokio_runtime().block_on(async move { discord_res() });
}
