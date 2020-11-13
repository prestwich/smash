extern crate smash;

use smash::{
    targets::sha256::Sha256Precompile,
    fuzzer::Fuzzer,
};


fn main() {
    Fuzzer::new().run_against_control::<Sha256Precompile>(2);
}