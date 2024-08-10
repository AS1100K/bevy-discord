use std::sync::OnceLock;
use tokio::runtime::Runtime;

/// Tokio runtime, use this if you want to use async code inside bevy systems
pub fn tokio_runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}
