//! A Wrapper around [Http]

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::Resource;
use serenity::http::Http;
use std::sync::Arc;

/// Discord Http Plugin that inserts [DiscordHttpResource] to the app
pub struct DiscordHttpPlugin(&'static str);

impl DiscordHttpPlugin {
    /// Returns [DiscordHttpPlugin] generated with bot token
    pub fn new(token: &'static str) -> DiscordHttpPlugin {
        DiscordHttpPlugin(token)
    }
}

impl Plugin for DiscordHttpPlugin {
    fn build(&self, app: &mut App) {
        let http: Arc<Http> = Arc::new(Http::new(self.0));

        app.insert_resource(DiscordHttpResource { http });
    }
}

/// A Bevy Resource that holds [Http]
#[derive(Resource)]
pub struct DiscordHttpResource {
    http: Arc<Http>,
}

impl DiscordHttpResource {
    pub fn client(&self) -> Arc<Http> {
        self.http.clone()
    }

    pub fn client_ref(&self) -> &Http {
        &self.http
    }
}
