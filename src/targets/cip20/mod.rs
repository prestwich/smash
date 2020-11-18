use lain::{byteorder::ByteOrder, prelude::*};
use sha3::Digest;
use std::io::Write;

pub mod blake2s;

use blake2s::Blake2sGenOpts;


use crate::traits::{Target, TargetWithControl, ThreadContext, ProduceInvalid};

const SHA_3_256_SELECTOR: u8 = 0x00;
const SHA_3_512_SELECTOR: u8 = 0x01;
const KECCAK_512_SELECTOR: u8 = 0x02;

// Update whenever a new variant is added
const VARIANT_COUNT: u8 = 4;

#[derive(Debug)]
pub enum CIP20Modes {
    Sha3_256(Vec<u8>),
    Sha3_512(Vec<u8>),
    Keccak512(Vec<u8>),
    Blake2s(Blake2sGenOpts),
}

impl BinarySerialize for CIP20Modes {
    fn binary_serialize<W: Write, E: ByteOrder>(&self, buf: &mut W) -> usize {
        match self {
            CIP20Modes::Sha3_256(preimage) => {
                buf.write_all(&[SHA_3_256_SELECTOR]).unwrap();
                buf.write_all(preimage).unwrap();
                preimage.len() + 1
            }
            CIP20Modes::Sha3_512(preimage) => {
                buf.write_all(&[SHA_3_512_SELECTOR]).unwrap();
                buf.write_all(preimage).unwrap();
                preimage.len() + 1
            }
            CIP20Modes::Keccak512(preimage) => {
                buf.write_all(&[KECCAK_512_SELECTOR]).unwrap();
                buf.write_all(preimage).unwrap();
                preimage.len() + 1
            }
            CIP20Modes::Blake2s(opts) => {
                opts.binary_serialize::<W, E>(buf)
            }
        }
    }
}

impl NewFuzzed for CIP20Modes {
    type RangeType = u8;
    fn new_fuzzed<R: Rng>(mutator: &mut Mutator<R>, constraints: Option<&Constraints<Self::RangeType>>) -> Self {

        let (min, max, weight) = {
            if let Some(range) = constraints {
                (range.min.unwrap_or(0),
                range.max.unwrap_or(VARIANT_COUNT),
                range.weighted)
            } else {
                (0, VARIANT_COUNT, Default::default())
            }
        };

        let choice: u8 = mutator.gen_weighted_range(min, max, weight);
        match choice {
            0 => CIP20Modes::Sha3_256(mutator.gen()),
            1 => CIP20Modes::Sha3_512(mutator.gen()),
            2 => CIP20Modes::Keccak512(mutator.gen()),
            3 => CIP20Modes::Blake2s(Blake2sGenOpts::Valid(mutator.gen())),
            _ => panic!("unreachable"),
        }
    }
}

pub struct Cip20Precompile;

impl Target for Cip20Precompile {
    type Intermediate = CIP20Modes;
    type Rng = lain::rand::rngs::StdRng;

    fn new() -> Cip20Precompile {
        Self
    }

    fn name() -> &'static str {
        "cip20"
    }

    fn run_experimental(
        &mut self,
        context: &mut ThreadContext,
        input: &[u8],
    ) -> Vec<Result<Vec<u8>, String>> {
        vec![context.celo.run_precompile(0xf3, input)]
    }
}

impl ProduceInvalid for Cip20Precompile {
    fn generate_invalid(&self, mutator: &mut Mutator<Self::Rng>) -> Self::Intermediate {
        CIP20Modes::Blake2s(Blake2sGenOpts::Invalid(mutator.gen()))
    }
}

impl TargetWithControl for Cip20Precompile {
    fn run_control(&self, input: &Self::Intermediate) -> Result<Vec<u8>, String> {
        match input {
            CIP20Modes::Sha3_256(buf) => Ok(sha3::Sha3_256::digest(buf).to_vec()),
            CIP20Modes::Sha3_512(buf) => Ok(sha3::Sha3_512::digest(buf).to_vec()),
            CIP20Modes::Keccak512(buf) => Ok(sha3::Keccak512::digest(buf).to_vec()),
            CIP20Modes::Blake2s(
                Blake2sGenOpts::Valid(opts)
            ) => Ok(opts.run()),
            _ => Err("Error: unknown".to_owned())
        }
    }
}
