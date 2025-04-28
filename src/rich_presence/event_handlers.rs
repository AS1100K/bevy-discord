use crate::common::{send_event, send_event_tuple};
use crate::events::{rich_presence::*, EventCollectionRichPresence};
use async_trait::async_trait;
use discord_sdk::activity::events::{InviteEvent, JoinRequestEvent, SecretEvent};
use discord_sdk::overlay::events::UpdateEvent as OverlayUpdateEvent;
use discord_sdk::user::events::{ConnectEvent, UpdateEvent};
use discord_sdk::{DiscordHandler, DiscordMsg, Event};
use flume::Sender;
use tracing::error;

pub struct EventHandler {
    pub tx: Sender<EventCollectionRichPresence>,
}

#[async_trait]
impl DiscordHandler for EventHandler {
    async fn on_message(&self, msg: DiscordMsg) {
        match msg {
            DiscordMsg::Error(err) => error!("Got an error from `discord-sdk` i.e. feature `rich-presence` in `bevy-discord`. Error => {:?}", err),
            DiscordMsg::Event(event) => match event {
                Event::Ready(ConnectEvent {version, user, ..}) => send_event!(self, EventCollectionRichPresence, RichPresenceReady { version, user }),
                Event::Disconnected { reason } => send_event!(self, EventCollectionRichPresence, RichPresenceDisconnected { reason }),
                Event::CurrentUserUpdate(UpdateEvent { user }) => send_event!(self, EventCollectionRichPresence, RichPresenceCurrentUserUpdate { user }),
                Event::ActivityJoin(SecretEvent { secret }) => send_event!(self, EventCollectionRichPresence, RichPresenceActivityJoin { secret }),
                Event::ActivitySpectate(SecretEvent { secret }) => send_event!(self, EventCollectionRichPresence, RichPresenceActivitySpectate { secret }),
                Event::ActivityJoinRequest(JoinRequestEvent { user }) => send_event!(self, EventCollectionRichPresence, RichPresenceActivityJoinRequest { user }),
                Event::ActivityInvite(InviteEvent (activity_invite)) => send_event_tuple!(self, EventCollectionRichPresence, RichPresenceActivityInvite(activity_invite)),
                Event::OverlayUpdate(OverlayUpdateEvent { enabled, visible }) => send_event!(self, EventCollectionRichPresence, RichPresenceOverlayUpdate { enabled, visible }),
                Event::RelationshipUpdate(relationship) => send_event_tuple!(self, EventCollectionRichPresence, RichPresenceRelationshipUpdate(relationship)),
                _ => send_event_tuple!(self, EventCollectionRichPresence, RichPresenceError(event))
            }
        }
    }
}
