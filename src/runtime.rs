/// Returns a reference to a lazily-initialised, library-managed Tokio
/// [`Runtime`](tokio::runtime::Runtime).
///
/// Use this when you need to spawn async work from a Bevy system and you do
/// not have (or do not want) your own Tokio runtime (e.g. no `#[tokio::main]`
/// in your binary).
///
/// This function is only available when the `tokio_runtime` feature is
/// enabled.  The `rich_presence` feature enables it automatically; for the
/// `bot` feature you must opt-in explicitly.
///
/// # Panics
///
/// Panics if Tokio is unable to build the runtime (extremely unlikely in
/// practice).
#[cfg(feature = "tokio_runtime")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio_runtime")))]
pub fn tokio_runtime() -> &'static tokio::runtime::Runtime {
    use std::sync::OnceLock;
    static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}
