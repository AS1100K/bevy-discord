// examples/reactions.rs
use bevy::prelude::*;
use bevy_discord::bot::serenity::all::*;
use bevy_discord::bot::{events::*, DiscordBotConfig, DiscordBotPlugin};
use serde_json::json;

fn main() {
    let config = DiscordBotConfig::default()
        .token("YOUR_BOT_TOKEN_HERE")
        .gateway_intents(
            GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MESSAGE_REACTIONS,
        );

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(DiscordBotPlugin::new(config))
        .add_systems(Update, (handle_messages, handle_reactions))
        .run();
}

fn handle_messages(
    mut messages: EventReader<BMessage>,
    http: Res<bevy_discord::http::DiscordHttpResource>,
) {
    for message in messages.read() {
        if message.new_message.author.bot {
            continue;
        }

        if message.new_message.content == "!react" {
            let http = http.client();
            let message_id = message.new_message.id;
            let channel_id = message.new_message.channel_id;

            bevy_discord::runtime::tokio_runtime().spawn(async move {
                // Add a thumbs up reaction
                let _ = http
                    .create_reaction(
                        channel_id,
                        message_id,
                        &ReactionType::Unicode("👍".to_string()),
                    )
                    .await;
            });
        }
    }
}

fn handle_reactions(
    mut reaction_add: EventReader<BReactionAdd>,
    http: Res<bevy_discord::http::DiscordHttpResource>,
) {
    for reaction in reaction_add.read() {
        // Skip bot reactions
        if reaction.add_reaction.user_id == Some(reaction.ctx.cache.current_user().id) {
            continue;
        }

        let http = http.client();
        let channel_id = reaction.add_reaction.channel_id;

        bevy_discord::runtime::tokio_runtime().spawn(async move {
            let _ = http
                .send_message(
                    channel_id,
                    vec![],
                    &json!({
                        "content": "Thanks for reacting! 😊"
                    }),
                )
                .await;
        });
    }
}