use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=make.sh");

    let arg = "./make.sh";
    Command::new("sh").args(&[arg]).status().unwrap();

    Command::new("touch").args(&["build.rs"]).status().unwrap();
    Command::new("touch").args(&["make.sh"]).status().unwrap();
}
