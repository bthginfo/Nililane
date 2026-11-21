fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android") {
        // The prebuilt Oboe archive does not declare its C++ runtime to Cargo.
        println!("cargo:rustc-link-arg-cdylib=-lc++_static");
        println!("cargo:rustc-link-arg-cdylib=-lc++abi");
        println!("cargo:rustc-link-arg-cdylib=-Wl,--no-undefined");
        println!("cargo:rustc-link-arg-cdylib=-Wl,--wrap=AInputQueue_getEvent");
        println!("cargo:rustc-link-arg-cdylib=-Wl,-z,max-page-size=16384");
    }
}
