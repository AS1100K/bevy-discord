# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0-rc.1] - [Unreleased]

### Changed
- Hide functions available in `docsrs` feature
- `plugins` module is now private

### Fixed
- Typos in documentation

### Removed
- Re-exports of `DiscordHttpPlugin`, `DiscordBotPlugin`, `DiscordRichPresencePlugin`

## [0.6.0-alpha.3] - 2025-01-31

### Added
- feature `docsrs`
- `new` function implementation in `DiscordPluginGroup`
- `rich_presence` example in `README.md`

### Changed
- Moved `bevy_discord::rich_presence::discord_sdk` to `bevy_discord::discord_sdk`
- Made Re-exports directly instead of `pub use _::*` in module

### Fixed
- examples

## [0.6.0-alpha.2] - 2025-01-23

### Changed
- Made `setup_rich_presence` function use tokio's `block_on` function instead of spawn

  _This now ensures tokio runtime is available to `discord-sdk` crate when initializing `Disocrd`_

### Fixed
- Made docs buildable [See Error](https://docs.rs/crate/bevy-discord/0.6.0-alpha.1)

## [0.6.0-alpha.1] - 2025-01-06

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

## [0.5.0-beta.1] - 2024-11-28

### Added
- Added ability to have multiple shards in `DiscordBotConfig`

### Changed
- Upgraded bevy dependency from `0.14` to `0.15.0-rc.3`
- Made macros accessible to crate only - [60df93](https://github.com/AS1100K/bevy-discord/commit/60df9357c661a8bdc2caba39ce925f0e20b81b81)

### Removed
- Unused macros
