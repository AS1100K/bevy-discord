// examples/slash_commands.rs
// cargo run --example slash_command --features full

use bevy::prelude::*;
use bevy_discord::DiscordBotPlugin;
use bevy_discord::config::DiscordBotConfig;
use bevy_discord::messages::bot::*;
use bevy_discord::runtime::tokio_runtime;
use bevy_discord::serenity::all::{
    Command, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
    GatewayIntents,
};
use bevy_discord::serenity::model::application::Interaction;

fn main() {
    let config = DiscordBotConfig::default()
        .token("YOUR_BOT_TOKEN_HERE".to_string())
        .gateway_intents(GatewayIntents::GUILDS);

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(DiscordBotPlugin::new(config))
        .add_systems(Update, (handle_ready, handle_interactions))
        .run();
}

fn handle_ready(mut ready_events: MessageReader<BotReadyMessage>) {
    for event in ready_events.read() {
        let http = event.ctx.http.clone();

        // Register global slash command
        tokio_runtime().spawn(async move {
            let command = Command::create_global_command(
                &http,
                CreateCommand::new("ping").description("A simple ping command"),
            )
            .await;

            if let Err(why) = command {
                println!("Error creating command: {:?}", why);
            }
        });
    }
}

fn handle_interactions(mut interaction_events: MessageReader<InteractionCreateMessage>) {
    for event in interaction_events.read() {
        if let Interaction::Command(command) = &event.interaction {
            if command.data.name.as_str() == "ping" {
                let http = event.ctx.http.clone();
                let command = command.clone();

                tokio_runtime().spawn(async move {
                    let _ = command
                        .create_response(
                            &http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content("Pong! 🏓"),
                            ),
                        )
                        .await;
                });
            }
        }
    }
}
