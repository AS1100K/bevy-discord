use bevy_ecs::prelude::*;

use crate::bot::DiscordBotRes;
use crate::bot::events::*;

pub(crate) fn handle_b_ready_event (
    mut discord_bot_res: ResMut<DiscordBotRes>,
    mut events: EventReader<BReadyEvent>,
) {
    for event in events.read() {
        let http_clone = event.ctx.http.clone();

        discord_bot_res.http = Some(http_clone);
    }
}