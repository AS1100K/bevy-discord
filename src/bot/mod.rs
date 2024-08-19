use std::future::Future;
use std::sync::Arc;

use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;
use common::{send_events, BEventCollection};
use flume::Receiver;
use serenity::all::*;

use event_handlers::*;
use events::*;

use crate::bot::handle::Handle;
use crate::bot::http::InteractHttp;
use crate::runtime::tokio_runtime;
use crate::{initialize_field_with_doc, override_field_with_doc, DiscordSet};

mod common;
mod event_handlers;
pub mod events;
mod handle;
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub mod http;

/// Re-export serenity
pub mod serenity {
    #[doc(hidden)]
    pub use serenity::*;
}

pub struct DiscordBotPlugin(DiscordBotConfig);

impl DiscordBotPlugin {
    pub fn new(configuration: DiscordBotConfig) -> Self {
        Self(configuration)
    }
}

impl Plugin for DiscordBotPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "bot_cache")]
        app.add_event::<CacheRead>().add_event::<ShardsReady>();

        #[cfg(feature = "http")]
        app.add_event::<InteractHttp>();

        app.insert_resource(self.0.clone())
            .add_event::<ReadyEvent>()
            .add_event::<CommandPermissionsUpdate>()
            .add_event::<AutoModerationRuleCreate>()
            .add_event::<AutoModerationRuleUpdate>()
            .add_event::<AutoModerationRuleDelete>()
            .add_event::<AutoModerationActionExecution>()
            .add_event::<ChannelCreate>()
            .add_event::<CategoryCreate>()
            .add_event::<CategoryDelete>()
            .add_event::<ChannelDelete>()
            .add_event::<ChannelPinUpdate>()
            .add_event::<ChannelUpdate>()
            .add_event::<GuildAuditLogEntryCreate>()
            .add_event::<GuildBanAddition>()
            .add_event::<GuildBanRemoval>()
            .add_event::<GuildCreate>()
            .add_event::<GuildDelete>()
            .add_event::<GuildEmojisUpdate>()
            .add_event::<GuildIntegrationsUpdate>()
            .add_event::<GuildMemberAddition>()
            .add_event::<GuildMemberRemoval>()
            .add_event::<GuildMemberUpdate>()
            .add_event::<GuildMembersChunk>()
            .add_event::<GuildRoleCreate>()
            .add_event::<GuildRoleDelete>()
            .add_event::<GuildRoleUpdate>()
            .add_event::<GuildStickersUpdate>()
            .add_event::<GuildUpdate>()
            .add_event::<InviteCreate>()
            .add_event::<InviteDelete>()
            .add_event::<Message>()
            .add_event::<MessageDelete>()
            .add_event::<MessageDeleteBulk>()
            .add_event::<MessageUpdate>()
            .add_event::<ReactionAdd>()
            .add_event::<ReactionRemove>()
            .add_event::<ReactionRemoveAll>()
            .add_event::<ReactionRemoveEmoji>()
            .add_event::<PresenceUpdate>()
            .add_event::<Resume>()
            .add_event::<ShardStageUpdate>()
            .add_event::<TypingStart>()
            .add_event::<UserUpdate>()
            .add_event::<VoiceServerUpdate>()
            .add_event::<VoiceStateUpdate>()
            .add_event::<VoiceChannelStatusUpdate>()
            .add_event::<WebhookUpdate>()
            .add_event::<InteractionCreate>()
            .add_event::<IntegrationCreate>()
            .add_event::<IntegrationUpdate>()
            .add_event::<StageInstanceCreate>()
            .add_event::<StageInstanceUpdate>()
            .add_event::<StageInstanceDelete>()
            .add_event::<ThreadCreate>()
            .add_event::<ThreadUpdate>()
            .add_event::<ThreadDelete>()
            .add_event::<ThreadListSync>()
            .add_event::<ThreadMemberUpdate>()
            .add_event::<ThreadMembersUpdate>()
            .add_event::<GuildScheduledEventCreate>()
            .add_event::<GuildScheduledEventUpdate>()
            .add_event::<GuildScheduledEventDelete>()
            .add_event::<GuildScheduledEventUserAdd>()
            .add_event::<GuildScheduledEventUserRemove>()
            .add_event::<EntitlementCreate>()
            .add_event::<EntitlementUpdate>()
            .add_event::<EntitlementDelete>()
            .add_event::<PollVoteAdd>()
            .add_event::<PollVoteRemove>()
            .add_event::<RateLimit>()
            .add_systems(Startup, setup_bot.in_set(DiscordSet))
            .add_systems(
                Update,
                (send_events, handle_b_ready_event)
                    .chain()
                    .in_set(DiscordSet),
            );
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
    #[must_use]
    initialize_field_with_doc!(token, String, "Sets the bot token.");
    #[must_use]
    initialize_field_with_doc!(
        gateway_intents,
        GatewayIntents,
        "Sets the bot [`GatewayIntents`]."
    );
    override_field_with_doc!(status, OnlineStatus, "Sets the initial status.");
    override_field_with_doc!(activity, ActivityData, "Sets the initial activity.");
}

/// A Global Resource for Discord Bot. This resource holds important things like [Http]
#[derive(Resource)]
pub struct DiscordBotRes {
    pub(crate) http: Option<Arc<Http>>,
    pub(crate) recv: Receiver<BEventCollection>,
}

impl DiscordBotRes {
    /// [Http] is available once [BReadyEvent] is triggered
    ///
    /// NOTE: Calling this function can be expensive as it returns a clone of [Http], you
    /// may want to use [http::InteractHttp]
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// # use bevy_ecs::prelude::*;
    /// # use bevy_discord::bot::DiscordBotRes;
    /// use bevy_discord::bot::serenity::all::*;
    ///
    /// fn send_message (
    ///     discord_bot_res: Res<DiscordBotRes>
    /// ) {
    ///     let http = discord_bot_res.get_http();
    ///
    ///     if let Some(h) = http {
    ///         // Do anything you want to do with Http
    ///     }
    /// }
    /// ```
    pub fn get_http(&self) -> Option<Arc<Http>> {
        self.http.clone()
    }
}

fn setup_bot(mut commands: Commands, discord_bot_config: Res<DiscordBotConfig>) {
    let (tx, rx) = flume::unbounded();

    commands.insert_resource(DiscordBotRes {
        http: None,
        recv: rx,
    });

    let mut client = Client::builder(
        &discord_bot_config.token,
        discord_bot_config.gateway_intents,
    )
    .event_handler(Handle { tx });

    let discord_bot_res_clone = discord_bot_config.clone();

    if let Some(status) = discord_bot_res_clone.status {
        client = client.status(status);
    }

    if let Some(activity) = discord_bot_res_clone.activity {
        client = client.activity(activity);
    }

    tokio_runtime().spawn(async move {
        client
            .await
            .expect("Unable to build discord Client")
            .start()
            .await
            .expect("Unable to run the discord Client");
    });
}
