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