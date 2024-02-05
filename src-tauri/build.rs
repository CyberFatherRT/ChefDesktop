fn main() {
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=main");
    println!("cargo:rustc-link-lib=akrypt");
    println!("cargo:rustc-link-lib=akrypt-base");
    tauri_build::build()
}
