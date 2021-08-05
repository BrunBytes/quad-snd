fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
    }
}
