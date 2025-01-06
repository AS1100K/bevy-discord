#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "bot")]
use crate::bot::{DiscordBotConfig, DiscordBotPlugin};
#[cfg(any(feature = "bot", feature = "rich_presence"))]
use crate::channel::ChannelPlugin;
#[cfg(any(feature = "bot", feature = "rich_presence"))]
use crate::plugins::ChannelListener;
#[cfg(feature = "rich_presence")]
use crate::rich_presence::{DiscordRichPresenceConfig, DiscordRichPresencePlugin};
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ecs::schedule::SystemSet;

#[cfg(feature = "bot")]
#[cfg_attr(docsrs, doc(cfg(feature = "bot")))]
pub mod bot;

mod common;

#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub mod http;

#[cfg(feature = "rich_presence")]
#[cfg_attr(docsrs, doc(cfg(feature = "rich_presence")))]
pub mod rich_presence;

/// Tokio runtime, use this if you want to use async code inside bevy systems
pub mod runtime;

#[cfg(any(feature = "bot", feature = "rich_presence"))]
pub(crate) mod channel;
#[cfg(any(feature = "bot", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "bot", feature = "rich_presence"))))]
pub mod events;
pub mod plugins;

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

#[cfg(any(feature = "bot", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "bot", feature = "rich_presence"))))]
pub struct DiscordPluginGroup {
    #[cfg(feature = "bot")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bot")))]
    pub discord_bot_config: DiscordBotConfig,
    #[cfg(feature = "rich_presence")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rich_presence")))]
    pub discord_rich_presence_config: DiscordRichPresenceConfig,
}

#[cfg(any(feature = "bot", feature = "rich_presence"))]
impl PluginGroup for DiscordPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        let mut plugin_group = PluginGroupBuilder::start::<Self>()
            .add(ChannelPlugin)
            .add(ChannelListener);

        #[cfg(feature = "bot")]
        {
            plugin_group = plugin_group.add(DiscordBotPlugin::new(self.discord_bot_config));
        }

        #[cfg(feature = "rich_presence")]
        {
            plugin_group = plugin_group.add(DiscordRichPresencePlugin::new(
                self.discord_rich_presence_config,
            ));
        }

        plugin_group
    }
}
