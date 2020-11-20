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

/// A fuzzing Target. It defines 1 or more experimental runs, and provides
/// generation routine
pub trait Target: Send + Sync + Default {
    type Intermediate: BinarySerialize + NewFuzzed;
    type Rng: Rng;

    /// A short human-readable name for the targt
    fn name() -> &'static str;

    /// Instantiate a new target (alias for Default)
    fn new() -> Self {
        Default::default()
    }

    /// Make 1 or more experimental runs with the given input. Typically this
    /// will be calling the Geth and Celo instances (present in the context
    /// object).
    fn run_experimental(
        &mut self,
        context: &mut ThreadContext,
        input: &[u8],
    ) -> Vec<CommunicationResult<Vec<u8>>>;

    /// Generate a new test case. This may be overriden with custom generation
    /// logic.
    fn generate(&self, mutator: &mut Mutator<Self::Rng>) -> Self::Intermediate {
        Self::Intermediate::new_fuzzed(mutator, None)
    }

    /// Generate a new test case and serialize it. Produces output suitable for
    /// calling `run_experimental`.
    fn generate_serialized(&self, mutator: &mut Mutator<Self::Rng>) -> Vec<u8> {
        let mut buf = vec![];
        self.generate(mutator)
            .binary_serialize::<_, lain::byteorder::BigEndian>(&mut buf);
        buf
    }

    /// Shortcut function to generate the next test case and run it immediately.
    fn run_next_experimental(
        &mut self,
        context: &mut ThreadContext,
        mutator: &mut Mutator<Self::Rng>,
    ) -> Vec<CommunicationResult<Vec<u8>>> {
        let buf = self.generate_serialized(mutator);
        self.run_experimental(context, &buf)
    }

    #[doc(hidden)]
    // Instantiate a new fuzzer object parameterized with this type.
    fn new_fuzzer() -> Fuzzer<Self>
    where
        Self: Sized,
    {
        Fuzzer::<Self>::new()
    }
}

pub trait ProduceInvalid: Target {
    /// Generate an invalid test case.
    fn generate_invalid(&self, mutator: &mut Mutator<Self::Rng>) -> Self::Intermediate;

    /// Shortcut function to generate a new invalid test case and serialize it.
    fn generate_serialized_invalid(&self, mutator: &mut Mutator<Self::Rng>) -> Vec<u8> {
        let mut buf = vec![];
        self.generate_invalid(mutator)
            .binary_serialize::<_, lain::byteorder::BigEndian>(&mut buf);
        buf
    }
}

pub trait TargetWithControl: Target {
    /// Run a control function against an input. Experimental results may be
    /// compared to this result.
    fn run_control(&self, input: &<Self as Target>::Intermediate) -> Result<Vec<u8>, String>;

    /// Shortcut function to generate a test case, and compare the experimental
    /// results to the control. This produces `ComparisonError`s identifying
    /// discrepancies. If the experimental run encouters an abnormal error
    /// (usually cause by panic in the underlying context), the result will be
    /// `ComparisonError::NoComp`.
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

    /// Shortcut function to generate a test case and immediately call compare.
    fn compare_next_experimental(
        &mut self,
        ctx: &mut ThreadContext,
        mutator: &mut Mutator<Self::Rng>,
    ) -> Vec<Result<(), ComparisonError>> {
        let case = self.generate(mutator);
        self.compare(ctx, &case)
    }
}
