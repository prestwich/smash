extern crate smash;

use smash::{targets::Sha256Precompile, traits::Target};

fn main() {
    Sha256Precompile::new_fuzzer(false).run_against_control(2);
}
