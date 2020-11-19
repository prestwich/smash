extern crate smash;

use smash::{cli, targets::Cip20Precompile};

fn main() {
    cli::produce_invalid_with_control::<Cip20Precompile>()
}
