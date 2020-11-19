use lain::{
    prelude::*,
    rand::Rng,
    traits::{BinarySerialize, NewFuzzed},
};

use crate::{
    call::Caller,
    errors::{CommunicationError, CommunicationResult, ComparisonError, ComparisonResult},
    fuzzer::Fuzzer,
};

pub struct ThreadContext {
    pub(crate) celo: Caller,
    pub(crate) geth: Caller,
}

impl Default for ThreadContext {
    fn default() -> Self {
        Self {
            celo: Caller::new_celo(),
            geth: Caller::new_geth(),
        }
    }
}

pub trait Target: Send + Sync + Default {
    type Intermediate: BinarySerialize + NewFuzzed;
    type Rng: Rng;

    fn name() -> &'static str;

    fn new() -> Self {
        Default::default()
    }


    fn run_experimental(
        &mut self,
        context: &mut ThreadContext,
        input: &[u8],
    ) -> Vec<CommunicationResult<Vec<u8>>>;

    // Ought to be overriden in most cases
    fn generate(&self, mutator: &mut Mutator<Self::Rng>) -> Self::Intermediate {
        Self::Intermediate::new_fuzzed(mutator, None)
    }

    fn generate_next(&self, mutator: &mut Mutator<Self::Rng>) -> Vec<u8> {
        let mut buf = vec![];
        self.generate(mutator)
            .binary_serialize::<_, lain::byteorder::BigEndian>(&mut buf);
        buf
    }

    fn run_next_experimental(
        &mut self,
        context: &mut ThreadContext,
        mutator: &mut Mutator<Self::Rng>,
    ) -> Vec<CommunicationResult<Vec<u8>>> {
        let buf = self.generate_next(mutator);
        self.run_experimental(context, &buf)
    }

    fn new_fuzzer() -> Fuzzer<Self>
    where
        Self: Sized,
    {
        Fuzzer::<Self>::new()
    }
}

pub trait ProduceInvalid: Target {
    fn generate_invalid(&self, mutator: &mut Mutator<Self::Rng>) -> Self::Intermediate;

    fn generate_next_invalid(&self, mutator: &mut Mutator<Self::Rng>) -> Vec<u8> {
        let mut buf = vec![];
        self.generate_invalid(mutator)
            .binary_serialize::<_, lain::byteorder::BigEndian>(&mut buf);
        buf
    }
}

pub trait TargetWithControl: Target {
    fn run_control(&self, input: &<Self as Target>::Intermediate) -> Result<Vec<u8>, String>;

    fn compare(
        &mut self,
        ctx: &mut ThreadContext,
        input: &<Self as Target>::Intermediate,
    ) -> Vec<ComparisonResult> {
        let mut buf = vec![];
        input.binary_serialize::<_, lain::byteorder::BigEndian>(&mut buf);
        let experimental = self.run_experimental(ctx, &buf);
        let control = self.run_control(input);

        experimental
            .into_iter()
            .map(|a| {
                let c = control.clone();

                match (a, c) {
                    (Ok(left), Ok(right)) => {
                        if left == right {
                            Ok(())
                        } else {
                            Err(ComparisonError::OkNotEqual(left, right))
                        }
                    }
                    (Err(CommunicationError::RemoteError(left)), Ok(right)) => {
                        Err(ComparisonError::LeftErr(left, right))
                    }
                    (Ok(left), Err(right)) => Err(ComparisonError::RightErr(left, right)),
                    (Err(CommunicationError::RemoteError(left)), Err(right)) => {
                        if left == right {
                            Ok(())
                        } else {
                            Err(ComparisonError::ErrNotEqual(left, right))
                        }
                    }
                    _ => Err(ComparisonError::NoComp),
                }
            })
            .collect()
    }

    fn compare_next_experimental(
        &mut self,
        ctx: &mut ThreadContext,
        mutator: &mut Mutator<Self::Rng>,
    ) -> Vec<Result<(), ComparisonError>> {
        let case = self.generate(mutator);
        self.compare(ctx, &case)
    }
}
