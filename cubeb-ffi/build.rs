extern crate cmake;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

macro_rules! t {
    ($e:expr) => (match $e{
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

fn main() {
    if !Path::new("cubeb/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init", "--recursive"]).status();
    }

    let _ = fs::remove_dir_all(env::var("OUT_DIR").unwrap());
    t!(fs::create_dir_all(env::var("OUT_DIR").unwrap()));

    let dst = cmake::Config::new("cubeb")
        .build_target("cubeb")
        .build();

    println!("cargo:rustc-link-lib=static=cubeb");
    println!("cargo:rustc-link-search=native={}/build", dst.display());
}
