use lain::traits::BinarySerialize;

use crate::{
    errors::CommunicationResult,
    traits::{Target, TargetWithControl, ThreadContext},
};

#[derive(Debug, Default)]
pub struct IdentityPrecompile;

impl Target for IdentityPrecompile {
    type Intermediate = Vec<u8>;
    type Rng = lain::rand::rngs::StdRng;
    type Config = ();

    fn name() -> &'static str {
        "identity"
    }

    fn run_raw(
        &mut self,
        context: &mut ThreadContext,
        input: &[u8],
    ) -> Vec<CommunicationResult<Vec<u8>>> {
        vec![
            context.geth.run_precompile(4u8, input),
            context.celo.run_precompile(4u8, input),
        ]
    }
}

impl TargetWithControl for IdentityPrecompile {
    fn run_control(&self, input: &Self::Intermediate) -> Result<Vec<u8>, String> {
        let mut buf = vec![];
        input.binary_serialize::<_, lain::byteorder::BigEndian>(&mut buf);
        Ok(buf)
    }
}
