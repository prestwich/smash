use crate::{
    geth,
    traits::{Target, TargetWithControl},
};

pub struct IdentityPrecompile();

impl Target for IdentityPrecompile {
    type Intermediate = Vec<u8>;
    type Rng = lain::rand::rngs::StdRng;

    fn new() -> Self {
        Self()
    }

    fn name() -> &'static str {
        "identity"
    }

    fn run_experimental(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        geth::run_precompile(4u8, input)
    }
}

impl TargetWithControl for IdentityPrecompile {
    fn run_control(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        Ok(input.to_vec())
    }
}


