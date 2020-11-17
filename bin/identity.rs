extern crate smash;

use smash::{fuzzer::Fuzzer, targets::IdentityPrecompile};

fn main() {
    Fuzzer::new().run_against_control::<IdentityPrecompile>(4);
}
