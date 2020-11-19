extern crate smash;

use smash::{fuzzer::Fuzzer, targets::Cip20Precompile};

fn main() {
    Fuzzer::<Cip20Precompile>::new().run_invalid(4);
}
