use lain::{byteorder::{ByteOrder, WriteBytesExt}, prelude::*};
use std::io::Write;

const SELECTOR: u8 = 0x11;

fn xof_digest_length_to_node_offset(node_offset: u64, xof_digest_length: u16) -> u64 {
    let mut xof_digest_length_bytes: [u8; 2] = [0; 2];
    (&mut xof_digest_length_bytes[..])
        .write_u16::<LittleEndian>(xof_digest_length)
        .unwrap();
    let offset = node_offset as u64
        | ((xof_digest_length_bytes[0] as u64) << 32)
        | ((xof_digest_length_bytes[1] as u64) << 40);
    offset
}

#[derive(Debug, Clone)]
pub struct Blake2XsArgs {
    pub preimage: Vec<u8>,
    pub hash_length: u8,
    pub fanout: u8,
    pub depth: u8,
    pub leaf_length: u32,
    pub node_offset: u32,
    pub xof_digest_length: u16,
    pub node_depth: u8,
    pub inner_length: u8,
    pub salt: [u8; 8],
    pub personalization: [u8; 8],
    pub desired: u16,
    pub key: Vec<u8>,
}

impl Blake2XsArgs {
    pub fn run(&self) -> Vec<u8> {
        let h0 = blake2s_simd::Params::new()
            .hash_length(self.hash_length as usize)
            .fanout(self.fanout)
            .max_depth(self.depth)
            .max_leaf_length(self.leaf_length)
            .node_offset(xof_digest_length_to_node_offset(
                self.node_offset as u64,
                self.xof_digest_length,
            ))
            .node_depth(self.node_depth)
            .inner_hash_length(self.inner_length as usize)
            .salt(self.salt.as_ref())
            .personal(self.personalization.as_ref())
            .key(self.key.as_ref())
            .to_state()
            .update(self.preimage.as_ref())
            .finalize()
            .as_ref()
            .to_vec();


        let desired = if self.xof_digest_length < self.desired { self.xof_digest_length } else { self.desired };
        let num_hashes = ( desired + 32 - 1) / 32;
        let mut result = vec![];

        for i in 0..num_hashes {
            let digest = blake2s_simd::Params::new()
                .hash_length(32)
                .fanout(0)
                .max_depth(0)
                .max_leaf_length(0)
                .node_offset(xof_digest_length_to_node_offset(
                    i as u64,
                    self.xof_digest_length,
                ))
                .node_depth(0)
                .inner_hash_length(32)
                .salt(self.salt.as_ref())
                .personal(self.personalization.as_ref())
                .key(&[])
                .to_state()
                .update(h0.as_ref())
                .finalize();

            result.extend(digest.as_ref());
        }

        result
    }
}

#[derive(Debug, Clone)]
pub enum Blake2XsGenOpts {
    Valid(Blake2XsArgs),
    Invalid(Vec<u8>),
}

impl NewFuzzed for Blake2XsArgs {
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
            xof_digest_length: mutator.gen(),
            node_depth: mutator.gen(),
            inner_length: mutator.gen_range(1, 33),
            salt: mutator.gen(),
            personalization: mutator.gen(),
            desired: mutator.gen_range(1, 256),
            key,
        }
    }
}

impl BinarySerialize for Blake2XsArgs {
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
        written += buf.write(&self.node_offset.to_le_bytes()).unwrap();
        written += buf.write(&self.xof_digest_length.to_le_bytes()).unwrap();
        written += buf.write(&[self.node_depth, self.inner_length]).unwrap();
        written += buf.write(self.salt.as_ref()).unwrap();
        written += buf.write(self.personalization.as_ref()).unwrap();
        written += buf.write(self.key.as_ref()).unwrap();
        written += buf.write(&self.desired.to_be_bytes()).unwrap();
        written += buf.write(self.preimage.as_ref()).unwrap();
        written
    }
}

impl NewFuzzed for Blake2XsGenOpts {
    type RangeType = ();

    fn new_fuzzed<R: Rng>(
        mutator: &mut Mutator<R>,
        constraints: Option<&Constraints<Self::RangeType>>,
    ) -> Self {
        if constraints.is_some() && mutator.gen_chance(0.05) {
            return Blake2XsGenOpts::Invalid(mutator.gen());
        }
        Blake2XsGenOpts::Valid(mutator.gen())
    }
}

impl BinarySerialize for Blake2XsGenOpts {
    fn binary_serialize<W: Write, E: ByteOrder>(&self, buf: &mut W) -> usize {
        match self {
            Blake2XsGenOpts::Valid(args) => args.binary_serialize::<W, E>(buf),
            Blake2XsGenOpts::Invalid(v) => v.binary_serialize::<W, E>(buf),
        }
    }
}
