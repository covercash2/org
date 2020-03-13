use std::io;
use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, OrgError>;

#[derive(Debug)]
pub enum OrgError {
    IoError(io::Error),
    ParseError(usize, String),
    Unexpected(String),
}

impl From<io::Error> for OrgError {
    fn from(error: io::Error) -> OrgError {
	OrgError::IoError(error)
    }
}

impl fmt::Display for OrgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	match self {
	    OrgError::IoError(io_error) => write!(f, "{}", io_error),
	    OrgError::ParseError(line, msg) =>
		write!(f, "error parsing line: {}\n{}", line, msg),
	    OrgError::Unexpected(msg) =>
		write!(f, "unexpected error:\n{}", msg)
	}
    }
}

impl error::Error for OrgError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	match self {
	    OrgError::IoError(io_error) => Some(io_error),
	    _ => None
	}
    }
}
