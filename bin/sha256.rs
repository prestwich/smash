extern crate smash;

use smash::{cli, targets::Sha256Precompile};

fn main() {
    cli::target_with_control::<Sha256Precompile>()
}
