#![doc = include_str!("../README.md")]

use bevy_ecs::schedule::SystemSet;

#[cfg(feature = "webhook")]
pub mod webhook;
#[cfg(feature = "bot")]
pub mod bot;
mod runtime;

/// Bevy [`SystemSet`] that contains all system of this plugin.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscordSet;