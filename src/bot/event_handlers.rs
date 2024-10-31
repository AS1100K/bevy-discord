use bevy_ecs::prelude::*;

use crate::bot::events::*;
use crate::http::DiscordHttpResource;

pub(crate) fn handle_b_ready_event(mut events: EventReader<BReadyEvent>, mut commands: Commands) {
    for event in events.read() {
        let http_clone = event.ctx.http.clone();

        commands.insert_resource(DiscordHttpResource::new(http_clone));
    }
}
