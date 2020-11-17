use lain::{byteorder::ByteOrder, prelude::*};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Blake2sArgs {
    pub preimage: Vec<u8>,
    pub hash_length: u8,
    pub fanout: u8,
    pub depth: u8,
    pub leaf_length: u32,
    pub node_offset: u32,
    pub xof_digest_len: u16,
    pub node_depth: u8,
    pub inner_length: u8,
    pub salt: [u8; 8],
    pub personalization: [u8; 8],
    pub key: Vec<u8>,
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
            node_offset: mutator.gen(),
            xof_digest_len: mutator.gen_range(1, 33),
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
        let mut written = buf.write(&[0x10]).unwrap();
        written += buf
            .write(&[
                self.hash_length,
                self.key.len() as u8,
                self.fanout,
                self.depth,
            ])
            .unwrap();
        written += buf.write(&self.leaf_length.to_le_bytes()).unwrap();
        written += buf.write(&self.node_offset.to_le_bytes()).unwrap();
        written += buf.write(&self.xof_digest_len.to_le_bytes()).unwrap();
        written += buf.write(&[self.node_depth, self.inner_length]).unwrap();
        written += buf.write(self.salt.as_ref()).unwrap();
        written += buf.write(self.personalization.as_ref()).unwrap();
        written += buf.write(&self.key).unwrap();
        written += buf.write(self.preimage.as_ref()).unwrap();
        written
    }
}


impl NewFuzzed for Blake2sGenOpts {
    type RangeType = ();

    fn new_fuzzed<R: Rng>(mutator: &mut Mutator<R>, constraints: Option<&Constraints<Self::RangeType>>) -> Self {
        if constraints.is_some() && mutator.gen_chance(0.05) {
            return Blake2sGenOpts::Invalid(mutator.gen());
        }
        Blake2sGenOpts::Valid(mutator.gen())
    }
}

impl BinarySerialize for Blake2sGenOpts {
    fn binary_serialize<W: Write, E: ByteOrder>(&self, buf: &mut W) -> usize {
        match self {
            Blake2sGenOpts::Valid(args) => args.binary_serialize::<W, E>(buf),
            Blake2sGenOpts::Invalid(v) => v.binary_serialize::<W, E>(buf),
        }
    }
}