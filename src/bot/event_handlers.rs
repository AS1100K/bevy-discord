use bevy_ecs::prelude::*;

use crate::messages::bot::*;
use crate::res::DiscordHttpResource;

pub(crate) fn handle_b_ready_message(
    mut events: MessageReader<BotReadyMessage>,
    mut commands: Commands,
) {
    for event in events.read() {
        let http_clone = event.ctx.http.clone();

        commands.insert_resource(DiscordHttpResource::new(http_clone));
    }
}
