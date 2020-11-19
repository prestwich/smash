extern crate smash;

use smash::{cli, targets::IdentityPrecompile};

fn main() {
    cli::target_with_control::<IdentityPrecompile>()
}
