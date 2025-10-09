//! Resources for managing Discord HTTP client and Rich Presence functionality.
//!
//! This module provides two main resources:
//! - `DiscordHttpResource`: For handling Discord HTTP client operations
//! - `DiscordRichPresenceRes`: For managing Discord Rich Presence integration

use bevy_ecs::prelude::*;
use std::sync::Arc;

/// A Bevy resource that provides access to the Discord HTTP client.
///
/// This resource is automatically inserted into the Bevy app when using
/// `DiscordHttpPlugin`. It wraps Serenity's `Http` client in an `Arc`
/// for safe concurrent access.
///
/// # Examples
///
/// ```rust,no_run
/// use bevy_discord::res::DiscordHttpResource;
/// use std::sync::Arc;
/// use serenity::all::Http;
///
/// let http = Arc::new(Http::new("token"));
/// let resource = DiscordHttpResource::new(http);
///
/// // Get a cloned Arc reference
/// let client = resource.client();
///
/// // Get a reference to the client
/// let client_ref = resource.client_ref();
/// ```
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
#[derive(Resource)]
pub struct DiscordHttpResource {
    /// Arc-wrapped Serenity HTTP client for Discord API operations.
    pub http: Arc<serenity::all::Http>,
}

#[cfg(feature = "http")]
impl DiscordHttpResource {
    /// Creates a new `DiscordHttpResource` instance.
    ///
    /// # Arguments
    ///
    /// * `http` - An Arc-wrapped Serenity HTTP client
    pub fn new(http: Arc<serenity::all::Http>) -> Self {
        Self { http }
    }

    /// Returns a cloned Arc reference to the HTTP client.
    ///
    /// Use this method when you need to share ownership of the client
    /// across multiple systems or threads.
    pub fn client(&self) -> Arc<serenity::all::Http> {
        self.http.clone()
    }

    /// Returns a reference to the underlying HTTP client.
    ///
    /// Use this method when you only need temporary access to the client
    /// and don't need to share ownership.
    pub fn client_ref(&self) -> &serenity::all::Http {
        &self.http
    }
}

/// A global resource for managing Discord Rich Presence functionality.
///
/// This resource maintains the bot's Rich Presence state and provides access
/// to the Discord SDK client for updating presence information.
///
/// # Examples
///
/// ```rust,no_run
/// use bevy_discord::res::DiscordRichPresenceRes;
/// use std::sync::Arc;
/// use discord_sdk::Discord;
///
/// let discord = Arc::new(Discord::new(/* ... */));
/// let resource = DiscordRichPresenceRes::new(discord);
///
/// // Get a cloned Arc reference
/// let rp = resource.get_rp();
///
/// // Get a reference to the Discord client
/// let rp_ref = resource.get_rp_ref();
/// ```
#[cfg(feature = "rich_presence")]
#[cfg_attr(docsrs, doc(cfg(feature = "rich_presence")))]
#[derive(Resource)]
pub struct DiscordRichPresenceRes {
    /// Arc-wrapped Discord SDK client for Rich Presence operations.
    pub discord: Arc<discord_sdk::Discord>,
}

#[cfg(feature = "rich_presence")]
impl DiscordRichPresenceRes {
    /// Creates a new `DiscordRichPresenceRes` instance.
    ///
    /// # Arguments
    ///
    /// * `rp` - An Arc-wrapped Discord SDK client
    pub fn new(rp: Arc<discord_sdk::Discord>) -> Self {
        Self { discord: rp }
    }

    /// Returns a cloned Arc reference to the Discord SDK client.
    ///
    /// Use this when you need to share ownership of the client.
    pub fn get_rp(&self) -> Arc<discord_sdk::Discord> {
        self.discord.clone()
    }

    /// Returns a reference to the Discord SDK client.
    ///
    /// Use this when you only need temporary access to the client.
    pub fn get_rp_ref(&self) -> &discord_sdk::Discord {
        &self.discord
    }
}
