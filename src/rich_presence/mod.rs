// // Not Accessible Publicly

//! [Discord Rich Presence](https://discord.com/developers/docs/rich-presence/overview) Integration with bevy
//!
//! # NOTE before using
//! - This module depends on [discord-sdk](https://crates.io/crates/discord-sdk) crate
//! - Since, Discord Game SDK is undocumented, closed-source, publicly distributed library, it can
//!   result in introduction to breaking changes at any time in future by discord.

mod event_handlers;

use crate::DiscordSystems;
use crate::messages::{MessageCollectionRichPresence, rich_presence::*, send_events_rich_presence};
use crate::rich_presence::event_handlers::MessageHandler;
use crate::{channel::ChannelRes, runtime::tokio_runtime};
use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;
use discord_sdk::Discord;
use std::sync::Arc;

/// A plugin for integrating Discord Rich Presence with the Bevy game engine. You
/// can do anything that you can do with [Game SDK](https://discord.com/developers/docs/developer-tools/game-sdk).
///
/// # Development
///
/// For development following the following guides:
/// - [How can users discover and play my activity](https://support-dev.discord.com/hc/en-us/articles/21204493235991-How-Can-Users-Discover-and-Play-My-Activity)
///
/// and make sure you have enabled sharing your activities with others.
pub struct DiscordRichPresencePlugin(crate::config::DiscordRichPresenceConfig);

impl DiscordRichPresencePlugin {
    /// Creates a new `DiscordRichPresencePlugin` with the given configuration.
    pub fn new(discord_rich_presence_config: crate::config::DiscordRichPresenceConfig) -> Self {
        Self(discord_rich_presence_config)
    }
}

impl Plugin for DiscordRichPresencePlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = flume::unbounded::<MessageCollectionRichPresence>();
        let channel_res = ChannelRes { tx, rx };
        app.insert_resource(channel_res);

        app.insert_resource(self.0.clone())
            .add_message::<ErrorMessage>()
            .add_message::<RpReadyMessage>()
            .add_message::<DisconnectedMessage>()
            .add_message::<CurrentUserUpdateMessage>()
            .add_message::<ActivityJoinMessage>()
            .add_message::<ActivitySpectateMessage>()
            .add_message::<ActivityJoinRequestMessage>()
            .add_message::<ActivityInviteMessage>()
            .add_message::<OverlayUpdateMessage>()
            .add_message::<RelationshipUpdateMessage>()
            .add_systems(Startup, setup_rich_presence)
            .add_systems(Update, send_events_rich_presence.in_set(DiscordSystems));
    }
}

fn setup_rich_presence(
    mut commands: Commands,
    discord_rich_presence_config: Res<crate::config::DiscordRichPresenceConfig>,
    channel_res: Res<ChannelRes<MessageCollectionRichPresence>>,
) {
    let tx = channel_res.tx.clone();
    let event_handler = Box::new(MessageHandler { tx });

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
