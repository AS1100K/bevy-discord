use crate::events::EventCollection;
use bevy_ecs::prelude::Resource;
use bevy_app::{App, Plugin};
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
