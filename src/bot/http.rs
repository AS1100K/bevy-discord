use core::future::Future;
use core::marker::Send;
use std::sync::Arc;
use bevy_ecs::prelude::*;
use serenity::all::*;
use crate::bot::DiscordBotRes;
use crate::runtime::tokio_runtime;

pub type HandlerFn = fn(http: &Arc<Http>, commands: &mut Commands) -> Fut;
pub type Fut = dyn Future<Output=dyn Send + 'static>;

#[derive(Event)]
pub struct InteractHttp
{
    pub handler: HandlerFn
}

impl InteractHttp {
    pub fn new(handler: HandlerFn) -> Self {
        Self {
            handler
        }
    }
}

fn handle_interact_http(
    mut events: EventReader<InteractHttp>,
    discord_bot_res: Res<DiscordBotRes>,
    mut commands: Commands
) {
    for InteractHttp { handler } in events.read() {
        if let Some(http) = &discord_bot_res.http {
            tokio_runtime().spawn(async move {
                handler(http, &mut commands).await
            });
        }
    }
}