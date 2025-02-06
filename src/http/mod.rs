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
//!     .add_plugins(DiscordHttpPlugin::new("your-bot-token-here".to_string()))
//!     .run();
//! ```

use crate::res::DiscordHttpResource;
use bevy_app::{App, Plugin};
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
    /// let plugin = DiscordHttpPlugin::new("your-bot-token-here".to_string());
    /// ```
    pub fn new(token: String) -> DiscordHttpPlugin {
        DiscordHttpPlugin(token)
    }
}

impl Plugin for DiscordHttpPlugin {
    fn build(&self, app: &mut App) {
        let http: Arc<Http> = Arc::new(Http::new(&self.0));

        app.insert_resource(DiscordHttpResource::new(http));
    }
}
