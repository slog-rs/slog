//! WARNING: Formatting changes to this file will disrupt the line and column numbers,
//! triggering test failures.
//! You have been warned.

pub fn record_no_track() -> slog::Record<'static> {
    let args = Box::leak(Box::new(format_args!("problem!")));
    slog::record!(slog::Level::Warning, "", args, slog::b!())
}

#[track_caller]
pub fn record_track() -> slog::Record<'static> {
    let args = Box::leak(Box::new(format_args!("problem!")));
    slog::record!(slog::Level::Warning, "", args, slog::b!())
}
