use crate::bot::DiscordBotRes;
use crate::runtime::tokio_runtime;
use bevy_ecs::prelude::*;
use core::future::Future;
use core::marker::Send;
use serenity::all::*;
use std::sync::Arc;

pub type HandlerFn = fn(http: &Arc<Http>, commands: &mut Commands) -> Fut;
pub type Fut = dyn Future<Output = dyn Send + 'static>;

#[derive(Event)]
pub struct InteractHttp {
    pub handler: HandlerFn,
}

impl InteractHttp {
    pub fn new(handler: HandlerFn) -> Self {
        Self { handler }
    }
}

fn handle_interact_http(
    mut events: EventReader<InteractHttp>,
    discord_bot_res: Res<DiscordBotRes>,
    mut commands: Commands,
) {
    for InteractHttp { handler } in events.read() {
        if let Some(http) = &discord_bot_res.http {
            tokio_runtime().spawn(async move { handler(http, &mut commands).await });
        }
    }
}
