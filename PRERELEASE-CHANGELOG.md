# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0-alpha.1]

_This release note is continued from `v0.5` release note_

### Added
- `rich_presence` feature
- re-export `discord-sdk`
- macro `send_event_tuple`
- `ChannelPlugin`
- `ChannelListener`
- `DiscordPluginGroup`

### Changed
- move `bevy_discord::bot::events` to `bevy_discord::events::bot`
- Refactor `BEventCollection` to `EventCollection` and moved it from `bevy_discord::bot::common::BEventCollection` to `bevy_discord::events::EventCollection`
- Update `create_event_collection_and_handler` macro with refactored items

## [0.5.0-beta.1]

### Added
- Added ability to have multiple shards in `DiscordBotConfig`

### Changed
- Upgraded bevy dependency from `0.14` to `0.15.0-rc.3`
- Made macros accessible to crate only - [60df93](https://github.com/AS1100K/bevy-discord/commit/60df9357c661a8bdc2caba39ce925f0e20b81b81)

### Removed
- Unused macros