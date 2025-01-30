// examples/basic_bot.rs
use bevy::prelude::*;
use bevy_discord::bot::{DiscordBotConfig, DiscordBotPlugin};
use bevy_discord::events::bot::BMessage;
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
        .add_plugins(DiscordBotPlugin::new(config))
        .add_systems(Update, handle_messages)
        .run();
}

fn handle_messages(
    mut messages: EventReader<BMessage>,
    http: Option<Res<bevy_discord::http::DiscordHttpResource>>,
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
