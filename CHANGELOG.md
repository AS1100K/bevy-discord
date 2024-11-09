# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - Unreleased

### Added
- Added ability to have multiple shards in `DiscordBotConfig`

### Changed
- Upgraded bevy dependency from `0.13` to `0.15`
- Made macros accessible to crate only - [PR #6](https://github.com/AS1100K/bevy-discord/pulls/6)

### Removed
- Unused macros

## [0.4.0] - 2024-11-08

### Removed
- `webhook` module and feature

### Changed
- Add `DiscordHttpResource` once `BReadyEvent` is emitted in `DiscordBotPlugin`
- Upgraded bevy dependency from `0.13` to `0.14`

### Added
- Added Examples to `Cargo.toml`, so they can be scraped by cargo doc

## [0.3.1] - 2024-10-31

_For commit history see [https://github.com/AS1100K/bevy-discord/tree/v0.3.1](https://github.com/AS1100K/bevy-discord/tree/v0.3.1)_

## Changed
- Changed `DiscordBotConfig::token` type to `String` [#4](https://github.com/as1100k/bevy-discord/issues/4)

## [0.3.0] - 2024-10-31

### Added
- Module Http
- New Examples

### Changed
- Removed Explicit minor versioning of dependencies [#3](https://github.com/as1100k/bevy-discord/pull/3)
- Disabled default features for `reqwest` [#3](https://github.com/as1100k/bevy-discord/pull/3)
- Improved Documentation
- Moved `bevy_discord::bot::serenity` to `bevy_discord::serenity`

### Deprecated
- module `webhook`

## [0.2.2] - 2024-10-13

### Added
- Documentation regarding crate and various modules, structs and functions

### Changed
- License from `GPL-3` to `MIT`

### Fixes
- Removed Error when the channel is empty

## [0.2.1] - 2024-08-13

## Changed
- Made `token` accessible
- Made `gateway_intents` accessible

## [0.2.0] - 2024-08-12

### Added
- Re-export serenity
- Made `Http` accessible
- Added `DiscordBotPlugin` to `DiscordSet`

### Changed
- Made `BEventCollection` private
- Made `runtime` module public
- Replaced `default` feature with `full` feature

### Fixed
- Added feature on which a module is available in documentation.

## [0.2.0-beta.1] - 2024-08-10

### Added
- Integration with serenity
