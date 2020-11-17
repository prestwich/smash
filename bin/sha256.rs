extern crate smash;

use smash::{fuzzer::Fuzzer, targets::sha256::Sha256Precompile};

fn main() {
    Fuzzer::new().run_against_control::<Sha256Precompile>(2);
}
