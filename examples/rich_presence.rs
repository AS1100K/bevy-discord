// examples/rich_presence.rs
// cargo run --example rich_presence --features "full docsrs"

use bevy::log::tracing_subscriber::fmt::Subscriber;
use bevy::MinimalPlugins;
use bevy_app::{App, Update};
use bevy_discord::events::rich_presence::RichPresenceReady;
use bevy_discord::rich_presence::{DiscordRichPresenceConfig, DiscordRichPresenceRes};
use bevy_discord::{DiscordPluginGroup, DiscordSet};
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::{IntoSystemConfigs, Res};
use discord_sdk::activity::ActivityBuilder;
use discord_sdk::{OffsetDateTime, Subscriptions};

fn main() {
    // Initialize tracing subscriber
    let subscriber = Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config = DiscordRichPresenceConfig {
        app: 1326097363395411968,
        subscriptions: Subscriptions::all(),
    };

    App::new()
        .add_plugins(MinimalPlugins)
        // Don't use `::new_only_rich_presence` function in production code with feature `full` and `docsrs`
        // instead use `::new` with feature `rich_presence`
        .add_plugins(DiscordPluginGroup::new_only_rich_presence(config))
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
