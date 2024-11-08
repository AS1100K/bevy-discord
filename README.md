# Bevy Discord Plugin

![GitHub License](https://img.shields.io/github/license/AS1100K/bevy-discord)
![Crates.io Version](https://img.shields.io/crates/v/bevy-discord)
![CI](https://github.com/as1100k/bevy-discord/actions/workflows/ci.yml/badge.svg?event=push)

A very simple, bevy plugin that let you send and receive messages through bevy events.

## Installation

Add `bevy-discord` as your dependency:

```bash
$ cargo add bevy-discord --features full
```

## Quick Start

```rust,no_run
// examples/basic_bot.rs
use bevy::prelude::*;
use bevy_discord::serenity::all::*;
use bevy_discord::bot::{events::BMessage, DiscordBotConfig, DiscordBotPlugin};
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
    http: Res<bevy_discord::http::DiscordHttpResource>,
) {
    for message in messages.read() {
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
```
_Example taken from `examples/basic_bot.rs`_

## Examples

The `examples/` directory contains several example implementations:

- [`basic_bot.rs`](https://github.com/as1100k/bevy-discord/blob/main/examples/basic_bot.rs) - Simple message handling and response
- [`reactions.rs`](https://github.com/as1100k/bevy-discord/blob/main/examples/reactions.rs) - Handling reactions and emoji interactions
- [`slash_commands.rs`](https://github.com/as1100k/bevy-discord/blob/main/examples/slash_commands.rs) - Creating and handling slash commands

To run an example:

```bash
$ cargo run --example basic_bot
```

Note: Remember to replace `YOUR_BOT_TOKEN` with your actual Discord bot token.

## Features

This crate using powerful cargo features.

| Feature                   | Information                                                   |
|---------------------------|---------------------------------------------------------------|
| `bot` _(includes `http`)_ | Discord bot integration for Bevy applications.                |
| `http`                    | HTTP Client functionality for Discord API interactions.       |

_All features are comes under `full` feature._

## Limitations

Currently, the following Discord/Serenity features are not supported:

| Feature       | Module    |
|---------------|-----------|
| `voice`       | `bot`     |

## Versions
This crate aims to track bevy's versions. It also follows the semver standard. Below is a chart which versions of this
crate are compatible with which bevy version:

| Version                                                                                 | Bevy Version |
|-----------------------------------------------------------------------------------------|--------------|
| `0.2.x`                                                                                 | `0.13.x`     |
| `0.3.x`                                                                                 | `0.13.x`     |
| `0.4.x`                                                                                 | `0.14.x`     |
| `0.5.x` [On Branch `bevy-0.15`](https://github.com/AS1100K/bevy-discord/tree/bevy-0.15) | `0.15.x`     |
