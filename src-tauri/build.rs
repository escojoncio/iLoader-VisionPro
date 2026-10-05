fn main() {
    // Bigger main-thread stack on Windows (default is only 1 MB); see main.rs.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
            println!("cargo:rustc-link-arg-bins=/STACK:33554432");
        } else {
            println!("cargo:rustc-link-arg-bins=-Wl,--stack,33554432");
        }
    }
    tauri_build::build()
}
