//! Internal Channel Plugin, that just add
//! channel resource

use crate::{
    events::{send_events, EventCollection},
    DiscordSet,
};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{prelude::Resource, schedule::IntoSystemConfigs};
use flume::{Receiver, Sender};

pub struct ChannelPlugin;

#[derive(Resource)]
pub struct ChannelRes {
    pub rx: Receiver<EventCollection>,
    pub tx: Sender<EventCollection>,
}

impl Plugin for ChannelPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = flume::unbounded();

        app.insert_resource(ChannelRes { tx, rx });
    }
}

#[cfg(any(feature = "bot", feature = "rich_presence"))]
pub struct ChannelListener;

#[cfg(any(feature = "bot", feature = "rich_presence"))]
impl Plugin for ChannelListener {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_events.in_set(DiscordSet));
    }
}
