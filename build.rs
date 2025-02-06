fn main() {
    // Check if the "docsrs" feature is enabled
    if std::env::var_os("CARGO_FEATURE_DOCSRS").is_some() {
        println!(
            "cargo:warning=The 'docsrs' feature is intended for internal documentation \
            examples only and should not be used by end users."
        );
    }
}
