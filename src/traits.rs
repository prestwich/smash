use lain::{
    prelude::*,
    rand::Rng,
    traits::{BinarySerialize, NewFuzzed},
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ComparisonError {
    OkNotEqual(Vec<u8>, Vec<u8>),
    ErrNotEqual(String, String),
    LeftErr(String, Vec<u8>),
    RightErr(Vec<u8>, String),
    NoComp,
}

impl ComparisonError {
    fn strings(&self) -> (String, String) {
        match self {
            ComparisonError::OkNotEqual(left, right) => (hex::encode(&left), hex::encode(&right)),
            ComparisonError::ErrNotEqual(left, right) => (left.clone(), right.clone()),
            ComparisonError::LeftErr(left, right) => (left.clone(), hex::encode(&right)),
            ComparisonError::RightErr(left, right) => (hex::encode(left), right.clone()),
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for ComparisonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if *self == ComparisonError::NoComp {
            write!(f, "ComparisonError::NoComp")
        } else {
            let (left, right) = self.strings();
            writeln!(f, "ComparisonError {{")?;
            writeln!(f, "\tleft: {}", left)?;
            writeln!(f, "\tright: {}", right)?;
            writeln!(f, "}}")
        }
    }
}

pub trait Target: Send + Sync {
    type Intermediate: BinarySerialize + NewFuzzed;
    type Rng: Rng;

    fn new() -> Self;
    fn name() -> &'static str;

    fn run_experimental(&self, input: &[u8]) -> Result<Vec<u8>, String>;

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

    fn run_next_experimental(&self, mutator: &mut Mutator<Self::Rng>) -> Result<Vec<u8>, String> {
        let buf = self.generate_next(mutator);
        self.run_experimental(&buf)
    }
}

pub trait TargetWithControl: Target {
    fn run_control(&self, input: &[u8]) -> Result<Vec<u8>, String>;

    fn compare(&self, input: &[u8]) -> Result<(), ComparisonError> {
        let a = self.run_experimental(input);
        let b = self.run_control(input);

        match (a, b) {
            (Ok(left), Ok(right)) => {
                if left == right {
                    Ok(())
                } else {
                    Err(ComparisonError::OkNotEqual(left, right))
                }
            }
            (Err(left), Ok(right)) => Err(ComparisonError::LeftErr(left, right)),
            (Ok(left), Err(right)) => Err(ComparisonError::RightErr(left, right)),
            (Err(left), Err(right)) => {
                if left == right {
                    Ok(())
                } else {
                    Err(ComparisonError::ErrNotEqual(left, right))
                }
            }
        }
    }

    fn compare_next_experimental(
        &self,
        mutator: &mut Mutator<Self::Rng>,
    ) -> Result<(), ComparisonError> {
        let buf = self.generate_next(mutator);
        self.compare(&buf)
    }
}
