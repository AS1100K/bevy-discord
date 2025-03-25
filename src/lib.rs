#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "bot")]
use crate::plugins::ChannelListener;
use bevy_ecs::schedule::SystemSet;

#[cfg(feature = "bot")]
mod bot;
#[cfg(feature = "bot")]
pub use bot::DiscordBotPlugin;

#[cfg(any(feature = "bot", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "bot", feature = "rich_presence"))))]
pub mod config;
pub mod res;

mod common;

#[cfg(feature = "http")]
mod http;
#[cfg(feature = "http")]
pub use http::DiscordHttpPlugin;

#[cfg(feature = "rich_presence")]
mod rich_presence;
#[cfg(feature = "rich_presence")]
pub use rich_presence::DiscordRichPresencePlugin;

/// Tokio runtime, use this if you want to use async code inside bevy systems
pub mod runtime;

#[cfg(any(feature = "bot", feature = "rich_presence"))]
pub(crate) mod channel;
#[cfg(any(feature = "bot", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "bot", feature = "rich_presence"))))]
pub mod events;
mod plugins;

#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub use serenity;

#[cfg(feature = "rich_presence")]
#[cfg_attr(docsrs, doc(cfg(feature = "rich_presence")))]
pub use discord_sdk;

/// Bevy [`SystemSet`] that contains all system of this plugin.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscordSet;
