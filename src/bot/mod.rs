use std::sync::Arc;

use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;
use common::{send_events, BEventCollection};
use serenity::all::*;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use event_handlers::*;
use events::*;

use crate::bot::handle::Handle;
use crate::override_field_with_doc;
use crate::runtime::tokio_runtime;

mod handle;
pub mod events;
pub mod common;
mod event_handlers;

pub struct DiscordBotPlugin(DiscordBotConfig);

impl DiscordBotPlugin {
    pub fn new(configuration: DiscordBotConfig) -> Self {
        Self(configuration)
    }
}

impl Plugin for DiscordBotPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.0.clone())
            .add_event::<BReadyEvent>()
            .add_event::<BCommandPermissionsUpdate>()
            .add_event::<BAutoModerationRuleCreate>()
            .add_event::<BAutoModerationRuleUpdate>()
            .add_event::<BAutoModerationRuleDelete>()
            .add_event::<BAutoModerationActionExecution>()
            .add_event::<BChannelCreate>()
            .add_event::<BCategoryCreate>()
            .add_event::<BCategoryDelete>()
            .add_event::<BChannelDelete>()
            .add_event::<BChannelPinUpdate>()
            .add_event::<BChannelUpdate>()
            .add_event::<BGuildAuditLogEntryCreate>()
            .add_event::<BGuildBanAddition>()
            .add_event::<BGuildBanRemoval>()
            .add_event::<BGuildCreate>()
            .add_event::<BGuildDelete>()
            .add_event::<BGuildEmojisUpdate>()
            .add_event::<BGuildIntegrationsUpdate>()
            .add_event::<BGuildMemberAddition>()
            .add_event::<BGuildMemberRemoval>()
            .add_event::<BGuildMemberUpdate>()
            .add_event::<BGuildMembersChunk>()
            .add_event::<BGuildRoleCreate>()
            .add_event::<BGuildRoleDelete>()
            .add_event::<BGuildRoleUpdate>()
            .add_event::<BGuildStickersUpdate>()
            .add_event::<BGuildUpdate>()
            .add_event::<BInviteCreate>()
            .add_event::<BInviteDelete>()
            .add_event::<BMessage>()
            .add_event::<BMessageDelete>()
            .add_event::<BMessageDeleteBulk>()
            .add_event::<BMessageUpdate>()
            .add_event::<BReactionAdd>()
            .add_event::<BReactionRemove>()
            .add_event::<BReactionRemoveAll>()
            .add_event::<BReactionRemoveEmoji>()
            .add_event::<BPresenceUpdate>()
            .add_event::<BResume>()
            .add_event::<BShardStageUpdate>()
            .add_event::<BTypingStart>()
            .add_event::<BUserUpdate>()
            .add_event::<BVoiceServerUpdate>()
            .add_event::<BVoiceStateUpdate>()
            .add_event::<BVoiceChannelStatusUpdate>()
            .add_event::<BWebhookUpdate>()
            .add_event::<BInteractionCreate>()
            .add_event::<BIntegrationCreate>()
            .add_event::<BIntegrationUpdate>()
            .add_event::<BStageInstanceCreate>()
            .add_event::<BStageInstanceUpdate>()
            .add_event::<BStageInstanceDelete>()
            .add_event::<BThreadCreate>()
            .add_event::<BThreadUpdate>()
            .add_event::<BThreadDelete>()
            .add_event::<BThreadListSync>()
            .add_event::<BThreadMemberUpdate>()
            .add_event::<BThreadMembersUpdate>()
            .add_event::<BGuildScheduledEventCreate>()
            .add_event::<BGuildScheduledEventUpdate>()
            .add_event::<BGuildScheduledEventDelete>()
            .add_event::<BGuildScheduledEventUserAdd>()
            .add_event::<BGuildScheduledEventUserRemove>()
            .add_event::<BEntitlementCreate>()
            .add_event::<BEntitlementUpdate>()
            .add_event::<BEntitlementDelete>()
            .add_event::<BPollVoteAdd>()
            .add_event::<BPollVoteRemove>()
            .add_event::<BRateLimit>()
            .add_systems(Startup, setup_bot)
            // Issue is coming here
            .add_systems(Update, (send_events, handle_b_ready_event).chain());

        #[cfg(feature = "bot_cache")]
        app
            .add_event::<BCacheRead>()
            .add_event::<BShardsReady>();
    }
}

/// Configuration of discord bot
#[derive(Default, Resource, Clone)]
pub struct DiscordBotConfig {
    token: String,
    gateway_intents: GatewayIntents,
    status: Option<OnlineStatus>,
    activity: Option<ActivityData>,
}

impl DiscordBotConfig {
    override_field_with_doc!(status, OnlineStatus, "Sets the initial status.");
    override_field_with_doc!(activity, ActivityData, "Sets the initial activity.");
}

#[derive(Resource)]
pub struct DiscordBotRes {
    pub(crate) http: Option<Arc<Http>>,
    pub(crate) recv: Receiver<BEventCollection>,
}

fn setup_bot (
    mut commands: Commands,
    discord_bot_config: Res<DiscordBotConfig>,
) {
    let (tx, rx) = mpsc::channel(5);

    commands.insert_resource(DiscordBotRes {
        http: None,
        recv: rx,
    });

    let mut client = Client::builder(&discord_bot_config.token, discord_bot_config.gateway_intents).event_handler(Handle {
        tx
    });

    let discord_bot_res_clone = discord_bot_config.clone();

    if let Some(status) = discord_bot_res_clone.status {
        client = client.status(status);
    }

    if let Some(activity) = discord_bot_res_clone.activity {
        client = client.activity(activity);
    }

    tokio_runtime().spawn(async move {
        client
            .await.expect("Unable to build discord Client")
            .start()
            .await.expect("Unable to run the discord Client");
    });
}

