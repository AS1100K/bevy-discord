// // Not Accessible Publicly

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
use bevy_app::{App, Plugin, Startup};
use bevy_ecs::prelude::*;
use discord_sdk::Discord;
use std::sync::Arc;

/// TODO
pub struct DiscordRichPresencePlugin(crate::config::DiscordRichPresenceConfig);

impl DiscordRichPresencePlugin {
    pub fn new(discord_rich_presence_config: crate::config::DiscordRichPresenceConfig) -> Self {
        Self(discord_rich_presence_config)
    }
}

impl Plugin for DiscordRichPresencePlugin {
    fn build(&self, app: &mut App) {
        // Check if internal plugins are added
        // If you are adding new internal plugins, make sure to also update DiscordBotPlugin
        if app
            .get_added_plugins::<super::channel::ChannelPlugin>()
            .is_empty()
        {
            app.add_plugins(super::channel::ChannelPlugin);
        }
        if app.get_added_plugins::<super::ChannelListener>().is_empty() {
            app.add_plugins(super::ChannelListener);
        }

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
    discord_rich_presence_config: Res<crate::config::DiscordRichPresenceConfig>,
    channel_res: Res<ChannelRes>,
) {
    let tx = channel_res.tx.clone();
    let event_handler = Box::new(EventHandler { tx });

    let discord_rich_presence_config = discord_rich_presence_config.clone();
    let discord_res = || {
        commands.insert_resource(crate::res::DiscordRichPresenceRes::new(Arc::new(
            Discord::new(
                discord_rich_presence_config.app,
                discord_rich_presence_config.subscriptions,
                event_handler,
            )
            .expect("Failed to create a Discord Rich Presence Client"),
        )))
    };

    tokio_runtime().block_on(async move { discord_res() });
}
