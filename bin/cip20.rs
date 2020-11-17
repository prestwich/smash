extern crate smash;

use smash::{fuzzer::Fuzzer, targets::Cip20Precompile};

fn main() {
    Fuzzer::new().run_against_control::<Cip20Precompile>(4);
}
