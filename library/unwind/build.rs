use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET").expect("TARGET was not set");
    if target.contains("mos") {
        println!("cargo:rustc-link-lib=gcc_s");
    }
}

