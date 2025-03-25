// examples/reactions.rs
// cargo run --example reactions --features full

use bevy::prelude::*;
use bevy_discord::config::DiscordBotConfig;
use bevy_discord::events::bot::*;
use bevy_discord::serenity::all::*;
use bevy_discord::DiscordBotPlugin;
use serde_json::json;

fn main() {
    let config = DiscordBotConfig::default()
        .token("YOUR_BOT_TOKEN_HERE".to_string())
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
    http: Option<Res<bevy_discord::res::DiscordHttpResource>>,
) {
    for message in messages.read() {
        if let Some(http) = &http {
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
}

fn handle_reactions(
    mut reaction_add: EventReader<BReactionAdd>,
    http: Option<Res<bevy_discord::res::DiscordHttpResource>>,
) {
    for reaction in reaction_add.read() {
        if let Some(http) = &http {
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
}
