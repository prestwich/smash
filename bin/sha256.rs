extern crate smash;

use smash::{fuzzer::Fuzzer, targets::sha256::Sha256Precompile};

fn main() {
    Fuzzer::<Sha256Precompile>::new().run_against_control(2);
}
