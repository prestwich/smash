extern crate smash;

use smash::{targets::Cip20Precompile, traits::Target};

fn main() {
    Cip20Precompile::new_fuzzer().run_invalid(4);
}
