use thiserror::Error;

/// Result of communication to child process. IoError or a remote error string
pub type CommunicationResult<T> = Result<T, CommunicationError>;

pub type ComparisonResult = Result<(), ComparisonError>;

#[derive(Debug, Error)]
pub enum CommunicationError {
    /// Error using pipes. This usually means the child process has panicked
    #[error("Pipe read/write error: {0}")]
    IoError(#[from] std::io::Error),
    /// Child process returned an error string
    #[error("Remote call returned error message: {0}")]
    RemoteError(String),
}

#[derive(Debug, Eq, PartialEq, Clone, Error)]
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
            ComparisonError::OkNotEqual(left, right) => (
                "OkNotEqual".to_owned(),
                hex::encode(&left),
                hex::encode(&right),
            ),
            ComparisonError::ErrNotEqual(left, right) => {
                ("ErrNotEqual".to_owned(), wrap_err(left), wrap_err(right))
            }
            ComparisonError::LeftErr(left, right) => {
                ("LeftErr".to_owned(), wrap_err(left), hex::encode(&right))
            }
            ComparisonError::RightErr(left, right) => {
                ("RightErr".to_owned(), hex::encode(left), wrap_err(right))
            }
            ComparisonError::NoComp => ("NoComp".to_owned(), "".to_owned(), "".to_owned()),
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
            writeln!(f, "\tleft:  {}", left)?;
            writeln!(f, "\tright: {}", right)?;
            writeln!(f, "}}")
        }
    }
}
