#[cfg(any(feature = "bot", feature = "rich_presence"))]
use {
    crate::events::send_events,
    crate::DiscordSet,
    bevy_app::{App, Plugin, Update},
    bevy_ecs::prelude::IntoSystemConfigs,
};

#[cfg(feature = "bot")]
pub use crate::bot::DiscordBotPlugin;

#[cfg(feature = "rich_presence")]
pub use crate::rich_presence::DiscordRichPresencePlugin;

#[cfg(feature = "http")]
pub use crate::http::DiscordHttpPlugin;

#[cfg(any(feature = "bot", feature = "rich_presence"))]
pub struct ChannelListener;

#[cfg(any(feature = "bot", feature = "rich_presence"))]
impl Plugin for ChannelListener {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_events.in_set(DiscordSet));
    }
}
