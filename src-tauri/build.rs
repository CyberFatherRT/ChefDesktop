use cmake::Config;


fn main() {

    let _ = Config::new("../libakrypt")
        .out_dir("../libakrypt")
        .build_target("all")
        .build();

    println!("cargo:rustc-link-search=../libakrypt/build");
    println!("cargo:rustc-link-lib=static=akrypt");
    println!("cargo:rustc-link-lib=static=akrypt-base");

    tauri_build::build();
}
