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
}
