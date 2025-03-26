macro_rules! new {
    ($doc:expr $(, $field:ident : $type:ty)*) => {
        #[doc = $doc]
        pub fn new($($field: $type),*) -> Self {
            Self {
                $($field),*
            }
        }
    };
}

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
        $name:ident,
        $(
            $(#[$meta:meta])? $variant:ident
        ),* $(,)?
    ) => {
        // Define the enum with the provided variants
        #[allow(clippy::enum_variant_names)]
        pub(crate) enum $name {
            $(
                $(#[$meta])?
                $variant($variant),
            )*
        }

        pastey::paste! {
            #[derive(bevy_ecs::system::SystemParam)]
            pub(crate) struct [< $name SystemParam >]<'w> {
                $(
                    $(#[$meta])?
                    pub(crate) [< $variant:snake >]: bevy_ecs::event::EventWriter<'w, $variant>,
                )*
            }

            // Define the function to handle the events and send them through EventWriter
            pub(crate) fn send_events(
                channel: bevy_ecs::prelude::Res<$crate::channel::ChannelRes>,
                mut events_system_param: [< $name SystemParam >]
            ) {
                if let Ok(event) = channel.rx.try_recv() {
                    match event {
                        $(
                            $(#[$meta])?
                            $name::$variant(event_to_send) => {
                                events_system_param.[< $variant:snake >].send(event_to_send);
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
            $crate::events::EventCollection::$event($event {
                $($field),*
            })
        ).await {
            error!("Unable to send event to the channel")
        }
    };
}

macro_rules! send_event_tuple {
    ($self:ident, $event:ident ( $($field:ident),* )) => {
        if let Err(_) = $self.tx.send_async(
            $crate::events::EventCollection::$event($event ( $($field),* ))
        ).await {
            error!("Unable to send event to the channel")
        }
    };
}

pub(crate) use {
    create_event_collection_and_handler, initialize_field_with_doc, new, override_field_with_doc,
    send_event, send_event_tuple,
};
