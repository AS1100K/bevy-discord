//! Internal Channel Plugin, that just add
//! channel resource

use bevy_ecs::prelude::Resource;
use flume::{Receiver, Sender};

#[derive(Resource)]
pub struct ChannelRes<T> {
    pub rx: Receiver<T>,
    pub tx: Sender<T>,
}
