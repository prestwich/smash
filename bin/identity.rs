extern crate smash;

use smash::{targets::IdentityPrecompile, traits::Target};

fn main() {
    IdentityPrecompile::new_fuzzer().run(4);
}
