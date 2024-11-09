/// Creates a function that override the field which is `Option` and documentation
macro_rules! override_field_with_doc {
    ($name:ident, $type:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = Some($name);
            self
        }
    };
}

/// Creates a function that override/initialize a field with custom documentation
macro_rules! initialize_field_with_doc {
    ($name:ident, $type:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = $name;
            self
        }
    };
}

macro_rules! create_event_collection_and_handler {
    (
        $(
            $(#[$meta:meta])? $variant:ident
        ),* $(,)?
    ) => {
        // Define the enum with the provided variants
        #[allow(clippy::enum_variant_names)]
        pub(crate) enum BEventCollection {
            $(
                $(#[$meta])?
                $variant($variant),
            )*
        }

        // Define the function to handle the events and send them through EventWriter
        paste::paste! {
            pub(crate) fn send_events(
                world: &mut World
            ) {
                let discord_bot_res = world.resource::<$crate::bot::DiscordBotRes>();
                if let Ok(event) = discord_bot_res.recv.try_recv() {
                    match event {
                        $(
                            $(#[$meta])?
                            BEventCollection::$variant(event_to_send) => {
                                world.send_event(event_to_send);
                            }
                        ),*
                    }
                }
            }
        }
    };
}

macro_rules! send_event {
    ($self:ident, $event:ident { $($field:ident),* }) => {
        if let Err(_) = $self.tx.send_async(
            $crate::bot::common::BEventCollection::$event($event {
                $($field),*
            })
        ).await {
            error!("Unable to send event to the channel")
        }
    };
}

pub(crate) use {
    create_event_collection_and_handler, initialize_field_with_doc, override_field_with_doc,
    send_event,
};
