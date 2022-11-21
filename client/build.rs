extern crate pkg_config;
use std::{env, path::PathBuf};

fn main() {
    const LIB_DIR: &str = "distlib";

    pkg_config::Config::new()
        .statik(true)
        .probe("openssl")
        .unwrap();
    println!("cargo:rustc-link-lib=dylib={}", "crypto");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_dir = std::path::PathBuf::from(LIB_DIR).canonicalize().unwrap();

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;

    fs_extra::dir::copy(lib_dir, out_path, &options).unwrap();
}
