extern crate smash;

use smash::{targets::IdentityPrecompile, traits::Target};

fn main() {
    IdentityPrecompile::new_fuzzer(false).run(4);
}
