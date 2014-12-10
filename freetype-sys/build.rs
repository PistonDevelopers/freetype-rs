extern crate "pkg-config" as pkg_config;

fn main() {
    match pkg_config::find_library("freetype2") {
        Ok(_) => return,
        Err(_) => {
            if cfg!(windows) {
                println!("cargo:rustc-flags=-l freetype-6:dylib");
            } else {
                println!("cargo:rustc-flags=-l freetype:dylib");
            }
        }
    }
}
