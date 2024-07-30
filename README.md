# Bevy Discord Plugin

![GitHub License](https://img.shields.io/github/license/AS1100K/bevy-discord)
![Crates.io Version](https://img.shields.io/crates/v/bevy-discord)
![CI](https://github.com/as1100k/bevy-discord/actions/workflows/ci.yml/badge.svg?event=push)


A very simple, bevy plugin that let you send messages through discord webhooks. _In Future releases, this plugin will support 
discord applications & bots and can send & receive messages by them._

## Features
This crate using powerful cargo features.

| Feature   | Information                                                                             |
|-----------|-----------------------------------------------------------------------------------------|
| `webhook` | Uses discord webhooks, using this you can only send messages.                           |
| `bot`     | This uses `serenity` behind the scenes and you can create awesome discord bots with it. |

_both features are comes under `default` feature, so if you want to use any specific feature you need to pass `default-features=false`._

## Not Supported Features
Currently, this crate is under development and there are features that are supported by discord and serenity
but not supported by us.

| Feature       | Module    |
|---------------|-----------|
| `voice`       | `bot`     |
| `attachments` | `webhook` |

