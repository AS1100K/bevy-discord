use crate::common::{send_message, send_message_tuple};
use crate::messages::{MessageCollectionRichPresence, rich_presence::*};
use async_trait::async_trait;
use discord_sdk::activity::events::{InviteEvent, JoinRequestEvent, SecretEvent};
use discord_sdk::overlay::events::UpdateEvent as OverlayUpdateEvent;
use discord_sdk::user::events::{ConnectEvent, UpdateEvent};
use discord_sdk::{DiscordHandler, DiscordMsg, Event};
use flume::Sender;
use tracing::error;

pub struct MessageHandler {
    pub tx: Sender<MessageCollectionRichPresence>,
}

#[async_trait]
impl DiscordHandler for MessageHandler {
    async fn on_message(&self, msg: DiscordMsg) {
        match msg {
            DiscordMsg::Error(err) => error!(
                "Got an error from `discord-sdk` i.e. feature `rich-presence` in `bevy-discord`. Error => {:?}",
                err
            ),
            DiscordMsg::Event(event) => match event {
                Event::Ready(ConnectEvent { version, user, .. }) => {
                    send_message!(
                        self,
                        MessageCollectionRichPresence,
                        RpReadyMessage { version, user }
                    )
                }
                Event::Disconnected { reason } => {
                    send_message!(
                        self,
                        MessageCollectionRichPresence,
                        DisconnectedMessage { reason }
                    )
                }
                Event::CurrentUserUpdate(UpdateEvent { user }) => send_message!(
                    self,
                    MessageCollectionRichPresence,
                    CurrentUserUpdateMessage { user }
                ),
                Event::ActivityJoin(SecretEvent { secret }) => send_message!(
                    self,
                    MessageCollectionRichPresence,
                    ActivityJoinMessage { secret }
                ),
                Event::ActivitySpectate(SecretEvent { secret }) => send_message!(
                    self,
                    MessageCollectionRichPresence,
                    ActivitySpectateMessage { secret }
                ),
                Event::ActivityJoinRequest(JoinRequestEvent { user }) => send_message!(
                    self,
                    MessageCollectionRichPresence,
                    ActivityJoinRequestMessage { user }
                ),
                Event::ActivityInvite(InviteEvent(activity_invite)) => send_message_tuple!(
                    self,
                    MessageCollectionRichPresence,
                    ActivityInviteMessage(activity_invite)
                ),
                Event::OverlayUpdate(OverlayUpdateEvent { enabled, visible }) => send_message!(
                    self,
                    MessageCollectionRichPresence,
                    OverlayUpdateMessage { enabled, visible }
                ),
                Event::RelationshipUpdate(relationship) => send_message_tuple!(
                    self,
                    MessageCollectionRichPresence,
                    RelationshipUpdateMessage(relationship)
                ),
                _ => {
                    send_message_tuple!(self, MessageCollectionRichPresence, ErrorMessage(event))
                }
            },
        }
    }
}
