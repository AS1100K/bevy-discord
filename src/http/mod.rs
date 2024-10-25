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
///
/// ```rust
/// # use bevy_ecs::event::{Event, EventReader};
/// # use bevy_ecs::system::Res;
/// use serde_json::json;
/// use serenity::all::Change::ChannelId;
/// use bevy_discord::http::DiscordHttpResource;
/// use bevy_discord::runtime::tokio_runtime;
///
/// # #[derive(Event)]
/// # pub struct MessageTriggerEvent;
/// fn handle_send_message(
///     mut events: EventReader<MessageTriggerEvent>,
///     discord_http_resource: Res<DiscordHttpResource>
/// ) {
///     for event in events.read() {
///         tokio_runtime().spawn(async move {
///             let channel_id = ChannelId::new(123);
///             discord_http_resource.http
///                 .send_message(channel_id, vec![], &json!({
///                     "content": "Hello from bevy"
///                 }))
///                 .await
///                 .unwrap()
///         });
///     }
/// }
/// ```
#[derive(Resource)]
pub struct DiscordHttpResource {
    pub http: Arc<Http>,
}
