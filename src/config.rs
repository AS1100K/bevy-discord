use crate::common::{initialize_field_with_doc, override_field_with_doc};
use bevy_ecs::prelude::*;
use serenity::all::*;

/// Configuration settings for the Discord bot.
///
/// This struct allows you to configure various aspects of the bot including:
/// - Bot token
/// - Gateway intents
/// - Online status
/// - Activity status
#[cfg(feature = "bot")]
#[cfg_attr(docsrs, doc(cfg(feature = "bot")))]
#[derive(Default, Resource, Clone)]
pub struct DiscordBotConfig {
    pub(crate) token: String,
    pub(crate) gateway_intents: GatewayIntents,
    pub(crate) status: Option<OnlineStatus>,
    pub(crate) activity: Option<ActivityData>,
    pub(crate) shards: u32,
}

#[cfg(feature = "bot")]
impl DiscordBotConfig {
    initialize_field_with_doc!(token, String, "Sets the bot token.");
    initialize_field_with_doc!(
        gateway_intents,
        GatewayIntents,
        "Sets the bot [`GatewayIntents`]."
    );
    override_field_with_doc!(status, OnlineStatus, "Sets the initial status.");
    override_field_with_doc!(activity, ActivityData, "Sets the initial activity.");
    initialize_field_with_doc!(shards, u32, "The total number of shards to use.");
}

/// Configuration settings for Discord Rich Presence integration.
///
/// This struct allows configuring Rich Presence features including:
/// - Discord Application ID
/// - Rich Presence subscriptions
#[cfg(feature = "rich_presence")]
#[cfg_attr(docsrs, doc(cfg(feature = "rich_presence")))]
#[derive(Resource, Clone)]
pub struct DiscordRichPresenceConfig {
    pub(crate) app: discord_sdk::AppId,
    pub(crate) subscriptions: discord_sdk::Subscriptions,
}

#[cfg(feature = "rich_presence")]
impl Default for DiscordRichPresenceConfig {
    fn default() -> Self {
        Self {
            app: 0,
            subscriptions: discord_sdk::Subscriptions::all(),
        }
    }
}

#[cfg(feature = "rich_presence")]
impl DiscordRichPresenceConfig {
    initialize_field_with_doc!(app, discord_sdk::AppId, "Set the Discord Application ID.");
    initialize_field_with_doc!(
        subscriptions,
        discord_sdk::Subscriptions,
        "Set the subscription for Rich Presence"
    );
}
