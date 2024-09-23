use std::env;

fn main() {
    /*
     * NOTE(Techcable): For version-specific code use rustversion crate
     * instead of adding special-cased conditionals here.
     *
     * https://docs.rs/rustversion
     *
     * Examples:
     * #[cfg(macro_fix)] -> #[rustversion::since(1.42)]
     * #[cfg_attr(sane_version, allow(deprecated))]
     *     -> #[rustversion::attr(since(1.42), allow(deprecated))]
     *
     */
    let target = env::var("TARGET").unwrap();
    let is_emscripten = target == "asmjs-unknown-emscripten"
        || target == "wasm32-unknown-emscripten";

    if !is_emscripten {
        println!("cargo:rustc-cfg=integer128");
    }

    // In Rust 1.80, cfg names are validated at compile time
    // See blog: https://blog.rust-lang.org/2024/05/06/check-cfg.html
    //
    // On prior versions, this directive is ignored.
    println!("cargo:rustc-check-cfg=cfg(integer128)");
}
