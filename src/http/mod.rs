//! HTTP Client functionality for Discord API interactions.
//!
//! This module provides a Bevy plugin wrapper around Serenity's HTTP client,
//! allowing for easy integration of Discord HTTP capabilities into Bevy applications.
//!
//! # Example
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_discord::http::DiscordHttpPlugin;
//!
//! App::new()
//!     .add_plugins(DiscordHttpPlugin::new("your-bot-token-here"))
//!     .run();
//! ```

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::Resource;
use serenity::http::Http;
use std::sync::Arc;

/// A Bevy plugin that provides Discord HTTP functionality.
///
/// This plugin initializes a Discord HTTP client with the provided bot token
/// and makes it available throughout the application as a Bevy resource.
pub struct DiscordHttpPlugin(String);

impl DiscordHttpPlugin {
    /// Creates a new instance of the Discord HTTP plugin.
    ///
    /// # Arguments
    ///
    /// * `token` - A Discord bot token used for authentication
    ///
    /// # Example
    /// ```no_run
    /// use bevy_discord::http::DiscordHttpPlugin;
    ///
    /// let plugin = DiscordHttpPlugin::new("your-bot-token-here");
    /// ```
    pub fn new(token: String) -> DiscordHttpPlugin {
        DiscordHttpPlugin(token)
    }
}

impl Plugin for DiscordHttpPlugin {
    fn build(&self, app: &mut App) {
        let http: Arc<Http> = Arc::new(Http::new(&self.0));

        app.insert_resource(DiscordHttpResource { http });
    }
}

/// A Bevy resource that provides access to the Discord HTTP client.
///
/// This resource is automatically inserted into the Bevy app when using
/// [`DiscordHttpPlugin`]. It wraps Serenity's [`Http`] client in an [`Arc`]
/// for safe concurrent access.
#[derive(Resource)]
pub struct DiscordHttpResource {
    http: Arc<Http>,
}

impl DiscordHttpResource {
    /// Returns a cloned Arc reference to the HTTP client.
    ///
    /// Use this method when you need to share ownership of the client
    /// across multiple systems or threads.
    ///
    /// # Returns
    ///
    /// An [`Arc`]<[`Http`]> instance that can be safely cloned and shared.
    pub fn client(&self) -> Arc<Http> {
        self.http.clone()
    }

    /// Returns a reference to the underlying HTTP client.
    ///
    /// Use this method when you only need temporary access to the client
    /// and don't need to share ownership.
    ///
    /// # Returns
    ///
    /// A reference to the [`Http`] client.
    pub fn client_ref(&self) -> &Http {
        &self.http
    }
}
