extern crate smash;

use smash::{
    targets::IdentityPrecompile,
    fuzzer::Fuzzer,
};


fn main() {
    Fuzzer::new().run_against_control::<IdentityPrecompile>(4);
}