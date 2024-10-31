# Bevy Discord Plugin

![GitHub License](https://img.shields.io/github/license/AS1100K/bevy-discord)
![Crates.io Version](https://img.shields.io/crates/v/bevy-discord)
![CI](https://github.com/as1100k/bevy-discord/actions/workflows/ci.yml/badge.svg?event=push)

A very simple, bevy plugin that let you send and receive messages through bevy events.

## Usage

Add `bevy-discord` as your dependency:

```bash
$ cargo add bevy-discord --features full
```

Example Usage:

```rust
use bevy_ecs::prelude::*;
use bevy_discord::bot::{ 
    events::BMessage,
    DiscordBotRes,
    serenity::model::id::ChannelId
};
use bevy_discord::runtime::tokio_runtime;
use serde_json::json;

// Make sure to add [bevy_discord::bot::DiscordBotPlugin] to the `App`

#[allow(clippy::complexity)]
fn handle_chat_relay(
    // Event emitted when an message is received on discord
    mut events: EventReader<BMessage>,
    // Discord Res
    discord_bot_res: Res<DiscordBotRes>
) {
    for event in events.read() {
        let message_content = &event.new_message.content;
        
        println!("Got a new message -> {}", message_content);

        // ...
        // Using this message as some sort of command
        // ...
        
        // Send a message to discord
        let http = discord_bot_res.get_http().unwrap();

        tokio_runtime().spawn(async move {
            let channel_id = ChannelId::new(7);

            http.send_message(channel_id, Vec::new(), &json!({
                "content": "Hello from bevy-discord"
            })).await.unwrap();
        });
    }
}
```

_If you want real examples then, it's worth checking out crate 
[aether-core](https://github.com/AS1100K/aether/blob/main/aether-core/src/discord.rs), 
although it is archived now._

## Features

This crate using powerful cargo features.

| Feature                                                                                                       | Information                                                                             |
|---------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------|
| `bot`                                                                                                         | This uses `serenity` behind the scenes and you can create awesome discord bots with it. |
| `http`                                                                                                        | This is a wrapper around serenity `Http`. This mimics Discord API(s).                   |
| `webhook` (👎 Deprecated [Learn Why](https://github.com/AS1100K/bevy-discord/blob/main/deprecated-reason.md)) | Uses discord webhooks, using this you can only send messages.                           |

_All features are comes under `full` feature._

## Not Supported Features

Currently, this crate is under development and there are features that are supported by discord and serenity
but not supported by us.

| Feature       | Module    |
|---------------|-----------|
| `voice`       | `bot`     |
| `attachments` | `webhook` |

## Versions
This crate aims to track bevy's versions. It also follows the semver standard. Below is a chart which versions of this
crate are compatible with which bevy version:

| Version                          | Bevy Version |
|----------------------------------|--------------|
| `0.2.x`                          | `0.13.x`     |
| `0.3.x` _(Release Planned Soon)_ | `0.13.x`     |
