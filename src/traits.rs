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
    fn strings(&self) -> (String, String, String) {
        let wrap_err = |e: &str| -> String {
            let mut s = "Err:\t".to_owned();
            s.push_str(e);
            s
        };

        match self {
            ComparisonError::OkNotEqual(left, right) => {
                ("OkNotEqual".to_owned(), hex::encode(&left), hex::encode(&right))
            }
            ComparisonError::ErrNotEqual(left, right) => {
                ("ErrNotEqual".to_owned(), wrap_err(left), wrap_err(right))
            }
            ComparisonError::LeftErr(left, right) => {
                ("LeftErr".to_owned(), wrap_err(left), hex::encode(&right))
            }
            ComparisonError::RightErr(left, right) => {
                ("RightErr".to_owned(), hex::encode(left), wrap_err(right))
            }
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for ComparisonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if *self == ComparisonError::NoComp {
            write!(f, "\nComparisonError::NoComp")
        } else {
            let (variant, left, right) = self.strings();
            writeln!(f, "ComparisonError {} {{", variant)?;
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

    fn run_experimental(&mut self, input: &[u8]) -> Vec<Result<Vec<u8>, String>>;

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
        mutator: &mut Mutator<Self::Rng>,
    ) -> Vec<Result<Vec<u8>, String>> {
        let buf = self.generate_next(mutator);
        self.run_experimental(&buf)
    }
}

pub trait TargetWithControl: Target {
    fn run_control(&self, input: &[u8]) -> Result<Vec<u8>, String>;

    fn compare(&mut self, input: &[u8]) -> Vec<Result<(), ComparisonError>> {
        let experimental = self.run_experimental(input);
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
            })
            .collect()
    }

    fn compare_next_experimental(
        &mut self,
        mutator: &mut Mutator<Self::Rng>,
    ) -> Vec<Result<(), ComparisonError>> {
        let buf = self.generate_next(mutator);
        self.compare(&buf)
    }
}
