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

macro_rules! create_message_collection_and_handler {
    (
        $name:ident,
        $fn_name:ident,
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
                    pub(crate) [< $variant:snake >]: bevy_ecs::message::MessageWriter<'w, $variant>,
                )*
            }

            // Define the function to handle the events and send them through EventWriter
            pub(crate) fn [<send_events_ $fn_name>](
                channel: bevy_ecs::prelude::Res<$crate::channel::ChannelRes<$name>>,
                mut events_system_param: [< $name SystemParam >]
            ) {
                if let Ok(event) = channel.rx.try_recv() {
                    match event {
                        $(
                            $(#[$meta])?
                            $name::$variant(event_to_send) => {
                                events_system_param.[< $variant:snake >].write(event_to_send);
                            }
                        ),*
                    }
                }
            }
        }
    };
}

macro_rules! send_message {
    ($self:ident, $collection: ident, $event:ident { $($field:ident),* }) => {
        if let Err(_) = $self.tx.send_async(
            $crate::messages::$collection::$event($event {
                $($field),*
            })
        ).await {
            error!("Unable to send event to the channel")
        }
    };
}

macro_rules! send_message_tuple {
    ($self:ident, $collection: ident, $event:ident ( $($field:ident),* )) => {
        if let Err(_) = $self.tx.send_async(
            $crate::messages::$collection::$event($event ( $($field),* ))
        ).await {
            error!("Unable to send event to the channel")
        }
    };
}

// TODO: Should we use `allow(unused_imports)` as not all the macros are used with
// multiple feature combinations
pub(crate) use {
    create_message_collection_and_handler, initialize_field_with_doc, override_field_with_doc,
    send_message, send_message_tuple,
};
