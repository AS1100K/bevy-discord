// examples/basic_bot.rs
// cargo run --example basic_bot --features full

use bevy::prelude::*;
use bevy_discord::DiscordBotPlugin;
use bevy_discord::config::DiscordBotConfig;
use bevy_discord::messages::bot::DiscordMessage;
use bevy_discord::serenity::all::*;
use serde_json::json;

fn main() {
    // Configure the bot with necessary intents
    let config = DiscordBotConfig::default()
        .token("YOUR_BOT_TOKEN_HERE".to_string())
        .gateway_intents(
            GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILDS,
        );

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(bevy::log::LogPlugin {
            ..Default::default()
        })
        .add_plugins(DiscordBotPlugin::new(config))
        .add_systems(Update, handle_discord_message)
        .run();
}

fn handle_discord_message(
    mut messages: MessageReader<DiscordMessage>,
    http: Option<Res<bevy_discord::res::DiscordHttpResource>>,
) {
    for message in messages.read() {
        if let Some(http) = &http {
            // Skip messages from bots (including our own)
            if message.new_message.author.bot {
                continue;
            }

            let content = &message.new_message.content;
            let channel_id = message.new_message.channel_id;

            // Simple ping-pong command
            if content == "!ping" {
                let http = http.client();

                bevy_discord::runtime::tokio_runtime().spawn(async move {
                    let _ = http
                        .send_message(
                            channel_id,
                            vec![],
                            &json!({
                                "content": "Pong! 🏓"
                            }),
                        )
                        .await;
                });
            }
        }
    }
}
