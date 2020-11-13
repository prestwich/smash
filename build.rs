use std::process::Command;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=make.sh");

    let arg = "./make.sh";
    Command::new("sh").args(&[arg]).status().unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);

    println!("cargo:rustc-link-lib=static=callgeth");
    println!("cargo:rustc-link-lib=static=callcelo");
    Command::new("touch").args(&["build.rs"]).status().unwrap();
    Command::new("touch").args(&["make.sh"]).status().unwrap();
}