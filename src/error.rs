use clap::Error as ClapError;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;
use std::io::IntoInnerError;

#[derive(Debug)]
pub enum Error {
    Missing,
    Io(IoError),
    Clap(ClapError),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        return None;
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        return match self {
            Error::Missing => write!(fmt, "Missing options."),
            Error::Io(err) => write!(fmt, "{}", err),
            Error::Clap(err) => write!(fmt, "{}", err),
        };
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        return Error::Io(err.into());
    }
}

impl<W: Debug + Send + 'static> From<IntoInnerError<W>> for Error {
    fn from(err: IntoInnerError<W>) -> Self {
        return Error::Io(err.into());
    }
}

impl From<ClapError> for Error {
    fn from(err: ClapError) -> Self {
        return Error::Clap(err.into());
    }
}
