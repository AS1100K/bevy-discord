/// wrapper around `Self::default()`
#[macro_export]
macro_rules! new {
    () => {
        #[must_use]
        #[doc = "Creates a new empty struct."]
        pub fn new() -> Self {
            Self::default()
        }
    };
}

/// Creates a function that override the field which is `Option`
#[macro_export]
macro_rules! override_field {
    ($name:ident, $type:ty) => {
        #[doc = concat!("Adds `", stringify!($name), "` field.")]
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = Some($name);
            self
        }
    };
}

/// Creates a function that override the field which is `Option` and documentation
#[macro_export]
macro_rules! override_field_with_doc {
    ($name:ident, $type:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = Some($name);
            self
        }
    };
}

/// Creates a function that override/initialize a field
#[macro_export]
macro_rules! initialize_field {
    ($name:ident, $type:ty) => {
        #[doc = concat!("Adds `", stringify!($name), "` field.")]
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = $name;
            self
        }
    };
}

/// Creates a function that override/initialize a field with custom documentation
#[macro_export]
macro_rules! initialize_field_with_doc {
    ($name:ident, $type:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = $name;
            self
        }
    };
}

#[macro_export]
macro_rules! create_event_collection_and_handler {
    (
        $(
            $(#[$meta:meta])? $variant:ident
        ),* $(,)?
    ) => {
        // Define the enum with the provided variants
        pub enum BEventCollection {
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
                let discord_bot_res = world.resource::<crate::bot::DiscordBotRes>();
                if let Ok(event) = discord_bot_res.recv.try_recv() {
                    match event {
                        $(
                            $(#[$meta])?
                            BEventCollection::$variant(event_to_send) => {
                                world.send_event(event_to_send);
                            }
                        ),*
                    }
                } else {
                    tracing::error!("Unable to read event from the channel");
                }
            }
        }
    };
}

#[macro_export]
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