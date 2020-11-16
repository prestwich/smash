use crate::{
    traits::{Target, TargetWithControl, ThreadContext},
};

pub struct IdentityPrecompile;

impl Target for IdentityPrecompile {
    type Intermediate = Vec<u8>;
    type Rng = lain::rand::rngs::StdRng;

    fn new() -> IdentityPrecompile {
        Self
    }

    fn name() -> &'static str {
        "identity"
    }

    fn run_experimental(&mut self, context: &mut ThreadContext, input: &[u8]) -> Vec<Result<Vec<u8>, String>> {
        vec![
            context.geth.run_precompile(4u8, input),
            context.celo.run_precompile(4u8, input),
        ]
    }
}

impl TargetWithControl for IdentityPrecompile {
    fn run_control(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        Ok(input.to_vec())
    }
}

