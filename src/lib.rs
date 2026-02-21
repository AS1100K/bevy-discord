#![allow(unused_macros)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

use bevy_ecs::schedule::SystemSet;

#[cfg(feature = "bot")]
mod bot;
#[cfg(feature = "bot")]
#[cfg_attr(docsrs, doc(cfg(feature = "bot")))]
pub use bot::DiscordBotPlugin;

#[cfg(any(feature = "bot", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "bot", feature = "rich_presence"))))]
pub mod config;
#[cfg(any(feature = "http", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "http", feature = "rich_presence"))))]
pub mod res;

mod common;

#[cfg(feature = "http")]
mod http;
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub use http::DiscordHttpPlugin;

#[cfg(feature = "rich_presence")]
mod rich_presence;
#[cfg(feature = "rich_presence")]
#[cfg_attr(docsrs, doc(cfg(feature = "rich_presence")))]
pub use rich_presence::DiscordRichPresencePlugin;

/// Runtime utilities for running async code from Bevy systems.
///
/// When the `tokio_runtime` feature is enabled this module exposes
/// [`runtime::tokio_runtime()`] which returns a lazily-initialised,
/// library-managed [`tokio::runtime::Runtime`].  If you already have your own
/// Tokio runtime (e.g. via `#[tokio::main]`) you can omit the
/// `tokio_runtime` feature and use
/// [`tokio::runtime::Handle::current()`] directly instead.
pub mod runtime;

#[cfg(any(feature = "bot", feature = "rich_presence"))]
pub(crate) mod channel;
#[cfg(any(feature = "bot", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "bot", feature = "rich_presence"))))]
pub mod messages;

#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub use serenity;

#[cfg(feature = "rich_presence")]
#[cfg_attr(docsrs, doc(cfg(feature = "rich_presence")))]
pub use discord_sdk;

/// Bevy [`SystemSet`] that contains all system of this plugin.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscordSystems;
