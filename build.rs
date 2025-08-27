pub fn main() {
    if std::env::var_os("CARGO_FEATURE_STD").is_some()
        || rustversion::cfg!(since(1.81))
    {
        // TODO:: It would be nice to disallow use of std::error::Error
        // Renaming imports using the clippy-disallowed-types lint doesn't work
        println!("cargo:rustc-cfg=has_std_error")
    }
    if rustversion::cfg!(since(1.79)) {
        // use core::panic::Location::caller() rather than file!(), line!(), column!() macros.
        // This requires inline const { ... } expressions and Location::caller() to be a const-fn
        // Both are added in Rust 1.79
        println!("cargo:rustc-cfg=use_const_location");
    }
    if rustversion::cfg!(since(1.80)) {
        println!("cargo:rustc-check-cfg=cfg(use_const_location)");
        println!("cargo:rustc-check-cfg=cfg(has_std_error)")
    }
}
