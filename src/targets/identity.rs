use crate::{
    celo::Celo,
    geth::Geth,
    traits::{Target, TargetWithControl},
};

pub struct IdentityPrecompile(
    Celo,
    Geth
);

impl Target for IdentityPrecompile {
    type Intermediate = Vec<u8>;
    type Rng = lain::rand::rngs::StdRng;

    fn new() -> Self {
        Self(Celo::default(), Geth::default())
    }

    fn name() -> &'static str {
        "identity"
    }

    fn run_experimental(&mut self, input: &[u8]) -> Vec<Result<Vec<u8>, String>> {
        vec![
            self.0.run_precompile(4u8, input),
            self.1.run_precompile(4u8, input),
        ]
    }
}

impl TargetWithControl for IdentityPrecompile {
    fn run_control(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        Ok(input.to_vec())
    }
}

