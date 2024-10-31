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

## Examples

The `examples/` directory contains several example implementations:

- [`basic_bot.rs`](https://github.com/as1100k/bevy-discord/blob/main/examples/basic_bot.rs) - Simple message handling and response
- [`reactions.rs`](https://github.com/as1100k/bevy-discord/blob/main/examples/reactions.rs) - Handling reactions and emoji interactions
- [`slash_commands.rs`](https://github.com/as1100k/bevy-discord/blob/main/examples/slash_commands.rs) - Creating and handling slash commands

## Features

This crate using powerful cargo features.

| Feature                                                                                                       | Information                                                   |
|---------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------|
| `bot` _(includes `http`)_                                                                                     | Discord bot integration for Bevy applications.                |
| `http`                                                                                                        | HTTP Client functionality for Discord API interactions.       |
| `webhook` (👎 Deprecated [Learn Why](https://github.com/AS1100K/bevy-discord/blob/main/deprecated-reason.md)) | Uses discord webhooks, using this you can only send messages. |

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
