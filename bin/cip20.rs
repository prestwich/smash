extern crate smash;

use smash::{fuzzer::Fuzzer, targets::Cip20Precompile};

fn main() {
    Fuzzer::new().run_invalid::<Cip20Precompile>(4);
}
