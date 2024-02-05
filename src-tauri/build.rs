fn main() {
    println!("cargo:rustc-link-search={}/../libakrypt/build", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=akrypt");
    tauri_build::build()
}
