# Bevy Discord Plugin

![GitHub License](https://img.shields.io/github/license/AS1100K/bevy-discord)
[![Crates.io Version](https://img.shields.io/crates/v/bevy-discord)](https://crates.io/crates/bevy-discord)
![CI](https://github.com/as1100k/bevy-discord/actions/workflows/ci.yml/badge.svg?event=push)

A very simple, bevy plugin that let you send and receive messages through bevy events.

## Installation

Add `bevy-discord` as your dependency:

```bash
cargo add bevy-discord --features full
```

## Quick Start

<details>
<summary>Basic ping-pong bot</summary>

```rust,no_run
use bevy::prelude::*;
use bevy_discord::config::DiscordBotConfig;
use bevy_discord::events::bot::BMessage;
use bevy_discord::serenity::all::*;
use bevy_discord::DiscordBotPlugin;
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
        .add_systems(Update, handle_messages)
        .run();
}

fn handle_messages(
    mut messages: EventReader<BMessage>,
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
```

_Example taken from `examples/basic_bot.rs`_

</details>

<details>
<summary>Basic Rich-presence bot</summary>

```rust,no_run
use bevy::log::tracing_subscriber::fmt::Subscriber;
use bevy::prelude::*;
use bevy_discord::config::DiscordRichPresenceConfig;
use bevy_discord::events::rich_presence::RichPresenceReady;
use bevy_discord::res::DiscordRichPresenceRes;
use bevy_discord::{DiscordRichPresencePlugin, DiscordSet};
use discord_sdk::activity::ActivityBuilder;
use discord_sdk::OffsetDateTime;

fn main() {
    // Initialize tracing subscriber
    let subscriber = Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config = DiscordRichPresenceConfig::default()
        .app(1326097363395411968)
        .subscriptions(
            bevy_discord::discord_sdk::Subscriptions::ACTIVITY
                | bevy_discord::discord_sdk::Subscriptions::USER
                | bevy_discord::discord_sdk::Subscriptions::OVERLAY,
        );

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(DiscordRichPresencePlugin::new(config))
        .add_systems(Update, rich_presence_ready.after(DiscordSet))
        .run();
}

fn rich_presence_ready(
    mut events: EventReader<RichPresenceReady>,
    rich_presence: Res<DiscordRichPresenceRes>,
) {
    for event in events.read() {
        println!(
            r#"
            version: {},
            user: {:?}
            "#,
            event.version, event.user
        );

        println!("setup_rich_presence");
        let current_date_time = OffsetDateTime::now_utc();
        let new_activity = ActivityBuilder::new()
            .state("bevy-discord")
            .details("Exploring example rich_presence.rs")
            .start_timestamp(current_date_time)
            .kind(bevy_discord::discord_sdk::activity::ActivityKind::Playing);

        let ds = rich_presence.discord.clone();
        bevy_discord::runtime::tokio_runtime().spawn(async move {
            let _ = ds
                .update_activity(new_activity)
                .await
                .expect("Failed to update the activity");
        });
    }
}
```

_Example taken from `examples/rich_presence.rs`_

</details>

## Examples

The `examples/` directory contains several example implementations:

- [`basic_bot.rs`](https://github.com/as1100k/bevy-discord/blob/main/examples/basic_bot.rs) - Simple message handling and response
- [`reactions.rs`](https://github.com/as1100k/bevy-discord/blob/main/examples/reactions.rs) - Handling reactions and emoji interactions
- [`slash_commands.rs`](https://github.com/as1100k/bevy-discord/blob/main/examples/slash_commands.rs) - Creating and handling slash commands
- [`rich_presence.rs`](https://github.com/as1100k/bevy-discord/blob/main/examples/rich_presence.rs) - Simple bevy app which has Discord Rich Presence

To run an example:

```bash
cargo run --example <EXAMPLE_NAME> --features "full docsrs"
```

Note: Remember to replace `YOUR_BOT_TOKEN` with your actual Discord bot token.

## Features

This crate using powerful cargo features.

| Feature                   | Information                                                         |
|---------------------------|---------------------------------------------------------------------|
| `bot` _(includes `http`)_ | Discord bot integration for Bevy applications.                      |
| `http`                    | HTTP Client functionality for Discord API interactions.             |
| `rich_presence`           | Discord Rich Presence Integration with Bevy. _`(v0.6 and greater)`_ |

_All features are comes under `full` feature._

## Limitations

Currently, the following Discord/Serenity features are not supported:

| Feature       | Module    |
|---------------|-----------|
| `voice`       | `bot`     |

## Versions

This crate aims to track bevy's versions. It also follows the semver standard. Below is a chart which versions of this
crate are compatible with which bevy version:

| Version | Bevy Version |
|---------|--------------|
| `0.2.x` | `0.13.x`     |
| `0.3.x` | `0.13.x`     |
| `0.4.x` | `0.14.x`     |
| `0.5.x` | `0.15.x`     |
| `0.6.x` | `0.16.x`      |

## Contributing

If you are planning to contribute to `bevy-discord` in any manner, please refer to [`CONTRIBUTING.md`](https://github.com/AS1100K/bevy-discord/blob/main/CONTRIBUTING.md)
