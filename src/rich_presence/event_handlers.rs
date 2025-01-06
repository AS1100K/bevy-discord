use crate::common::{send_event, send_event_tuple};
use crate::events::{rich_presence::*, EventCollection};
use async_trait::async_trait;
use discord_sdk::activity::events::{InviteEvent, JoinRequestEvent, SecretEvent};
use discord_sdk::overlay::events::UpdateEvent as OverlayUpdateEvent;
use discord_sdk::user::events::{ConnectEvent, UpdateEvent};
use discord_sdk::{DiscordHandler, DiscordMsg, Event};
use flume::Sender;
use tracing::error;

pub struct EventHandler {
    pub tx: Sender<EventCollection>,
}

#[async_trait]
impl DiscordHandler for EventHandler {
    async fn on_message(&self, msg: DiscordMsg) {
        match msg {
            DiscordMsg::Error(err) => error!("Got an error from `discord-sdk` i.e. feature `rich-presence` in `bevy-discord`. Error => {:?}", err),
            DiscordMsg::Event(event) => match event {
                Event::Ready(ConnectEvent {version, user, ..}) => send_event!(self, RichPresenceReady { version, user }),
                Event::Disconnected { reason } => send_event!(self, RichPresenceDisconnected { reason }),
                Event::CurrentUserUpdate(UpdateEvent { user }) => send_event!(self, RichPresenceCurrentUserUpdate { user }),
                Event::ActivityJoin(SecretEvent { secret }) => send_event!(self, RichPresenceActivityJoin { secret }),
                Event::ActivitySpectate(SecretEvent { secret }) => send_event!(self, RichPresenceActivitySpectate { secret }),
                Event::ActivityJoinRequest(JoinRequestEvent { user }) => send_event!(self, RichPresenceActivityJoinRequest { user }),
                Event::ActivityInvite(InviteEvent (activity_invite)) => send_event_tuple!(self, RichPresenceActivityInvite(activity_invite)),
                Event::OverlayUpdate(OverlayUpdateEvent { enabled, visible }) => send_event!(self, RichPresenceOverlayUpdate { enabled, visible }),
                Event::RelationshipUpdate(relationship) => send_event_tuple!(self, RichPresenceRelationshipUpdate(relationship)),
                _ => send_event_tuple!(self, RichPresenceError(event))
            }
        }
    }
}
