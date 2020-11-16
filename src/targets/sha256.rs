use sha2::Digest;

use crate::{
    celo::Celo,
    geth::Geth,
    traits::{Target, TargetWithControl, ThreadContext}
};

pub struct Sha256Precompile(
    Celo,
    Geth
);

impl Target for Sha256Precompile {
    type Intermediate = Vec<u8>;
    type Rng = lain::rand::rngs::StdRng;

    fn new() -> Self {
        Self(Celo::default(), Geth::default())
    }

    fn name() -> &'static str {
        "sha256"
    }

    fn run_experimental(&mut self, ctx: &mut ThreadContext, input: &[u8]) -> Vec<Result<Vec<u8>, String>> {
        vec![
            ctx.geth.run_precompile(2u8, input),
            ctx.celo.run_precompile(2u8, input),
        ]
    }
}

impl TargetWithControl for Sha256Precompile {
    fn run_control(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        Ok(sha2::Sha256::digest(input).to_vec())
    }
}