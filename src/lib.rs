#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

use bevy_ecs::schedule::SystemSet;

#[cfg(feature = "bot")]
#[cfg_attr(docsrs, doc(cfg(feature = "bot")))]
pub mod bot;

mod common;

#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub mod http;

/// Tokio runtime, use this if you want to use async code inside bevy systems
pub mod runtime;

#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
/// Re-export serenity
pub mod serenity {
    #[doc(hidden)]
    pub use serenity::*;
}

/// Bevy [`SystemSet`] that contains all system of this plugin.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscordSet;
