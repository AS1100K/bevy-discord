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

pub mod res;

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

#[cfg(any(feature = "bot", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "bot", feature = "rich_presence"))))]
pub struct DiscordPluginGroup {
    #[cfg(feature = "bot")]
    /// _Only available in feature `bot`_
    pub bot_config: DiscordBotConfig,
    #[cfg(feature = "rich_presence")]
    /// _Only available in feature `rich_presence`_
    pub rich_presence_config: DiscordRichPresenceConfig,
}

#[cfg(all(feature = "bot", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "bot", feature = "rich_presence"))))]
impl DiscordPluginGroup {
    /// Creates a new `DiscordPluginGroup` with the specified bot and rich presence configurations.
    ///
    /// # Arguments
    ///
    /// * `bot_config` - Configuration for the Discord bot [_only on feature `bot`_].
    /// * `rich_presence_config` - Configuration for Discord rich presence [_only on feature `rich_presence`_].
    ///
    /// # Returns
    ///
    /// A new instance of `DiscordPluginGroup`.
    pub fn new(
        bot_config: DiscordBotConfig,
        rich_presence_config: DiscordRichPresenceConfig,
    ) -> Self {
        Self {
            bot_config,
            rich_presence_config,
        }
    }

    #[cfg(feature = "docsrs")]
    #[doc(hidden)]
    /// Creates a new `DiscordPluginGroup` with only `bot_config` and default `rich_presence_config`
    ///
    /// **NOTE:** This is an internal function only accessible by `docsrs` feature and is used
    /// for examples, don't use this feature in production code.
    pub fn new_only_bot(bot_config: DiscordBotConfig) -> Self {
        Self {
            bot_config,
            rich_presence_config: DiscordRichPresenceConfig {
                app: 0,
                subscriptions: discord_sdk::Subscriptions::all(),
            },
        }
    }

    #[cfg(feature = "docsrs")]
    #[doc(hidden)]
    /// Creates a new `DiscordPluginGroup` with only `rich_presence_config` and default `bot_config`
    ///
    /// **NOTE:** This is an internal function only accessible by `docsrs` feature and is used
    /// for examples, don't use this feature in production code.
    pub fn new_only_rich_presence(rich_presence_config: DiscordRichPresenceConfig) -> Self {
        Self {
            rich_presence_config,
            bot_config: Default::default(),
        }
    }
}

#[cfg(all(feature = "bot", not(feature = "rich_presence")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "bot", not(feature = "rich_presence")))))]
impl DiscordPluginGroup {
    /// Creates a new `DiscordPluginGroup` with the specified bot configurations.
    ///
    /// # Arguments
    ///
    /// * `bot_config` - Configuration for the Discord bot.
    ///
    /// # Returns
    ///
    /// A new instance of `DiscordPluginGroup`.
    pub fn new(bot_config: DiscordBotConfig) -> Self {
        Self { bot_config }
    }
}

#[cfg(all(feature = "rich_presence", not(feature = "bot")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "rich_presence", not(feature = "bot")))))]
impl DiscordPluginGroup {
    /// Creates a new `DiscordPluginGroup` with the specified rich_presence configurations.
    ///
    /// # Arguments
    ///
    /// * `rich_presence` - Configuration for Discord rich presence.
    ///
    /// # Returns
    ///
    /// A new instance of `DiscordPluginGroup`.
    pub fn new(rich_presence_config: DiscordRichPresenceConfig) -> Self {
        Self {
            rich_presence_config,
        }
    }
}

#[cfg(any(feature = "bot", feature = "rich_presence"))]
impl PluginGroup for DiscordPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        let mut plugin_group = PluginGroupBuilder::start::<Self>()
            .add(ChannelPlugin)
            .add(ChannelListener);

        #[cfg(feature = "bot")]
        {
            plugin_group = plugin_group.add(DiscordBotPlugin::new(self.bot_config));
        }

        #[cfg(feature = "rich_presence")]
        {
            plugin_group =
                plugin_group.add(DiscordRichPresencePlugin::new(self.rich_presence_config));
        }

        plugin_group
    }
}
