// examples/rich_presence.rs
// cargo run --example rich_presence --features full

use bevy::log::tracing_subscriber::fmt::Subscriber;
use bevy::MinimalPlugins;
use bevy_app::{App, Update};
use bevy_discord::config::DiscordRichPresenceConfig;
use bevy_discord::events::rich_presence::RichPresenceReady;
use bevy_discord::res::DiscordRichPresenceRes;
use bevy_discord::{DiscordRichPresencePlugin, DiscordSet};
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{IntoSystemConfigs, Res};
use discord_sdk::activity::ActivityBuilder;
use discord_sdk::OffsetDateTime;

fn main() {
    // Initialize tracing subscriber
    let subscriber = Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config = DiscordRichPresenceConfig::default().app(1326097363395411968);

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
            .start_timestamp(current_date_time);

        let ds = rich_presence.discord.clone();
        bevy_discord::runtime::tokio_runtime().spawn(async move {
            let _ = ds
                .update_activity(new_activity)
                .await
                .expect("Failed to update the activity");
        });
    }
}
