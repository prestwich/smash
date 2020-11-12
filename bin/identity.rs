extern crate smash;

use smash::{
    targets::IdentityPrecompile,
    fuzzer,
};


fn main() {
    fuzzer::run_against_control::<IdentityPrecompile>(2);
}