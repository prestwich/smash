use sha2::Digest;
use crate::{
    // celo,
    geth,
    traits::{Target, TargetWithControl}
};

pub struct Sha256Precompile;

impl Target for Sha256Precompile {
    type Intermediate = Vec<u8>;
    type Rng = lain::rand::rngs::StdRng;

    fn new() -> Self {
        Self
    }

    fn name() -> &'static str {
        "sha256"
    }

    fn run_experimental(&self, input: &[u8]) -> Vec<Result<Vec<u8>, String>> {
        vec![
            geth::run_precompile(2u8, input),
            // celo::run_precompile(2u8, input),
        ]
    }
}

impl TargetWithControl for Sha256Precompile {
    fn run_control(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        Ok(sha2::Sha256::digest(input).to_vec())
    }
}