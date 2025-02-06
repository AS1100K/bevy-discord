#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "bot")]
use crate::bot::DiscordBotPlugin;
#[cfg(any(feature = "bot", feature = "rich_presence"))]
use crate::channel::ChannelPlugin;
#[cfg(any(feature = "bot", feature = "rich_presence"))]
use crate::plugins::ChannelListener;
#[cfg(feature = "rich_presence")]
use crate::rich_presence::DiscordRichPresencePlugin;
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ecs::schedule::SystemSet;

#[cfg(feature = "bot")]
mod bot;

#[cfg(any(feature = "bot", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "bot", feature = "rich_presence"))))]
pub mod config;
pub mod res;

mod common;

#[cfg(feature = "http")]
mod http;

#[cfg(feature = "rich_presence")]
mod rich_presence;

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

#[cfg(any(feature = "bot", feature = "rich_presence", feature = "http"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "bot", feature = "rich_presence", feature = "http")))
)]
pub struct DiscordPluginGroup {
    #[cfg(feature = "bot")]
    /// _Only available in feature `bot`_
    pub bot_config: crate::config::DiscordBotConfig,
    #[cfg(feature = "rich_presence")]
    /// _Only available in feature `rich_presence`_
    pub rich_presence_config: crate::config::DiscordRichPresenceConfig,
    #[cfg(any(all(feature = "http", not(feature = "bot")), feature = "docsrs"))]
    /// _Only available in feature `http` and ~`bot`~_
    pub token: String,
}

#[cfg(all(feature = "bot", feature = "rich_presence"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "bot", feature = "rich_presence"))))]
impl DiscordPluginGroup {
    #[cfg(all(feature = "docsrs", feature = "bot", feature = "rich_presence"))]
    /// Creates a new instance of `DiscordPluginGroup`.
    ///
    /// **NOTE:** This function arguments depends on the feature flag that is
    /// enabled. See the following table:
    ///
    /// | Feature            | Arguments              |
    /// |--------------------|------------------------|
    /// | `bot`              | `bot_config`           |
    /// | `rich_presence`    | `rich_presence_config` |
    /// | `http` and ~`bot`~ | `token`                |
    ///
    /// _Feature `full` includes both `bot`, and `rich_presence` feature_
    ///
    /// _Feature `bot` includes `http` feature_
    ///
    /// ## Arguments
    ///
    /// - `bot_config`: [DiscordBotConfig](crate::config::DiscordBotConfig)
    /// - `rich_presence_config`: [DiscordRichPresenceConfig](crate::config::DiscordRichPresenceConfig)
    /// - `token`: [String], Discord Token _Can only send messages_
    ///
    /// ## Resources Available
    ///
    /// Once this plugin group is added to the App, it makes the configuration struct
    /// i.e. `DiscordBotConfig` and `DiscorsRichPresenceConfig` available.
    ///
    /// The following are the resources availabe in `res` module are created after the
    /// Startup schedule. Refer to the table below for more information.
    ///
    /// | Feature         | Resource                                                       |
    /// |-----------------|----------------------------------------------------------------|
    /// | `bot` or `http` | [`DiscordHttpResource`](crate::res::DiscordHttpResource)       |
    /// | `rich_presence` | [`DiscordRichPresenceRes`](crate::res::DiscordRichPresenceRes) |
    pub fn new(
        bot_config: crate::config::DiscordBotConfig,
        rich_presence_config: crate::config::DiscordRichPresenceConfig,
        token: String,
    ) -> Self {
        Self {
            bot_config,
            rich_presence_config,
            token,
        }
    }

    #[cfg(feature = "docsrs")]
    #[doc(hidden)]
    /// Creates a new `DiscordPluginGroup` with only `bot_config` and default `rich_presence_config`
    ///
    /// **NOTE:** This is an internal function only accessible by `docsrs` feature and is used
    /// for examples, don't use this feature in production code.
    pub fn new_only_bot(bot_config: crate::config::DiscordBotConfig) -> Self {
        Self {
            bot_config,
            rich_presence_config: crate::config::DiscordRichPresenceConfig::default(),
            token: String::new(),
        }
    }

    #[cfg(feature = "docsrs")]
    #[doc(hidden)]
    /// Creates a new `DiscordPluginGroup` with only `rich_presence_config` and default `bot_config`
    ///
    /// **NOTE:** This is an internal function only accessible by `docsrs` feature and is used
    /// for examples, don't use this feature in production code.
    pub fn new_only_rich_presence(
        rich_presence_config: crate::config::DiscordRichPresenceConfig,
    ) -> Self {
        Self {
            rich_presence_config,
            bot_config: Default::default(),
            token: String::new(),
        }
    }
}

#[cfg(any(feature = "bot", feature = "rich_presence", feature = "http"))]
impl DiscordPluginGroup {
    // feature only `bot`
    #[cfg(all(feature = "bot", not(feature = "rich_presence")))]
    common::new!(
        "Creates a new `DiscordPluginGroup`. _Available only on `bot` feature._\n\nFor more information, please refer to https://docs.rs/bevy-discord",
        bot_config: crate::config::DiscordBotConfig
    );

    // feature only `http`
    #[cfg(all(feature = "http", not(feature = "rich_presence"), not(feature = "bot")))]
    common::new!(
        "Creates a new `DiscordPluginGroup`. _Available only on `http` feature._\n\nFor more information, please refer to https://docs.rs/bevy-discord",
        token: String
    );

    // feature `http` and `rich_presence`
    #[cfg(all(feature = "http", feature = "rich_presence", not(feature = "bot")))]
    common::new!(
        "Creates a new `DiscordPluginGroup`. _Available only on `http` + `rich_presence` features._\n\nFor more information, please refer to https://docs.rs/bevy-discord",
        token: String,
        rich_presence_config: crate::config::DiscordRichPresenceConfig,
    );

    // feature `bot` and `rich_presence`
    #[cfg(all(feature = "bot", feature = "rich_presence", not(feature = "docsrs")))]
    common::new!(
        "Creates a new `DiscordPluginGroup`. _Available only on `bot` + `rich_presence` features._\n\nFor more information, please refer to https://docs.rs/bevy-discord",
        bot_config: crate::config::DiscordBotConfig,
        rich_presence_config: crate::config::DiscordRichPresenceConfig,
    );
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

        #[cfg(all(feature = "http", not(feature = "bot")))]
        {
            plugin_group = plugin_group.add(http::DiscordHttpPlugin(self.token));
        }

        plugin_group
    }
}
