use lain::{byteorder::ByteOrder, prelude::*};
use std::io::Write;

const SELECTOR: u8 = 0x10;

#[derive(Debug, Clone)]
pub struct Blake2sArgs {
    pub preimage: Vec<u8>,
    pub hash_length: u8,
    pub fanout: u8,
    pub depth: u8,
    pub leaf_length: u32,
    pub node_offset: u64,
    pub node_depth: u8,
    pub inner_length: u8,
    pub salt: [u8; 8],
    pub personalization: [u8; 8],
    pub key: Vec<u8>,
}

impl Blake2sArgs {
    pub fn run(&self) -> Vec<u8> {
        blake2s_simd::Params::new()
            .hash_length(self.hash_length as usize)
            .fanout(self.fanout)
            .max_depth(self.depth)
            .max_leaf_length(self.leaf_length)
            .node_offset(self.node_offset)
            .node_depth(self.node_depth)
            .inner_hash_length(self.inner_length as usize)
            .salt(self.salt.as_ref())
            .personal(self.personalization.as_ref())
            .key(self.key.as_ref())
            .to_state()
            .update(self.preimage.as_ref())
            .finalize()
            .as_ref()
            .to_vec()
    }
}

#[derive(Debug, Clone)]
pub enum Blake2sGenOpts {
    Valid(Blake2sArgs),
    Invalid(Vec<u8>),
}

impl NewFuzzed for Blake2sArgs {
    type RangeType = ();

    fn new_fuzzed<R: Rng>(
        mutator: &mut Mutator<R>,
        _: Option<&Constraints<Self::RangeType>>,
    ) -> Self {
        let mut key = mutator.gen::<Vec<u8>>();
        key.truncate(32);
        Self {
            preimage: mutator.gen(),
            hash_length: mutator.gen_range(1, 33),
            fanout: mutator.gen(),
            depth: mutator.gen(),
            leaf_length: mutator.gen(),
            node_offset: mutator.gen_range(0, 281474976710655), // 2 ** 48 - 1
            node_depth: mutator.gen(),
            inner_length: mutator.gen_range(1, 33),
            salt: mutator.gen(),
            personalization: mutator.gen(),
            key,
        }
    }
}

impl BinarySerialize for Blake2sArgs {
    fn binary_serialize<W: Write, E: ByteOrder>(&self, buf: &mut W) -> usize {
        let mut written = buf.write(&[SELECTOR]).unwrap();
        written += buf
            .write(&[
                self.hash_length,
                self.key.len() as u8,
                self.fanout,
                self.depth,
            ])
            .unwrap();
        written += buf.write(&self.leaf_length.to_le_bytes()).unwrap();
        written += buf.write(&self.node_offset.to_le_bytes()[..6]).unwrap();
        written += buf.write(&[self.node_depth, self.inner_length]).unwrap();
        written += buf.write(self.salt.as_ref()).unwrap();
        written += buf.write(self.personalization.as_ref()).unwrap();
        written += buf.write(self.key.as_ref()).unwrap();
        written += buf.write(self.preimage.as_ref()).unwrap();
        written
    }
}

impl NewFuzzed for Blake2sGenOpts {
    type RangeType = ();

    fn new_fuzzed<R: Rng>(
        mutator: &mut Mutator<R>,
        _constraints: Option<&Constraints<Self::RangeType>>,
    ) -> Self {
        Blake2sGenOpts::Valid(mutator.gen())
    }
}

impl BinarySerialize for Blake2sGenOpts {
    fn binary_serialize<W: Write, E: ByteOrder>(&self, buf: &mut W) -> usize {
        match self {
            Blake2sGenOpts::Valid(args) => args.binary_serialize::<W, E>(buf),
            Blake2sGenOpts::Invalid(v) => {
                let mut written = buf.write(&[0x11]).unwrap();
                written += v.binary_serialize::<W, E>(buf);
                written
            }
        }
    }
}
