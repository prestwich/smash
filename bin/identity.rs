extern crate smash;

use smash::{fuzzer::Fuzzer, targets::IdentityPrecompile};

fn main() {
    Fuzzer::<IdentityPrecompile>::new().run_against_control(4);
}
