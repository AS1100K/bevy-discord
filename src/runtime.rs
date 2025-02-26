#[cfg(any(feature = "tokio_runtime", feature = "rich_presence"))]
pub fn runtime() -> &'static tokio::runtime::Runtime {
    static RUNTIME: std::sync::OnceLock<tokio::runtime::Runtime> = std::sync::OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}

#[cfg(all(feature = "bevy_tasks_runtime", not(feature = "rich_presence")))]
pub fn runtime() -> &'static bevy_tasks::IoTaskPool {
    bevy_tasks::IoTaskPool::get()
}
