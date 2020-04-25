use std::convert::From;
use std::fmt;
use std::io;
use std::net;
use std::num;

#[derive(Debug)]
pub enum Error<I> {
    Io(io::Error),
    Num(num::ParseIntError),
    IpAddr(net::AddrParseError),
    DateTime(chrono::format::ParseError),
    Parse((I, nom::error::ErrorKind)),
    Other(Box<dyn std::error::Error>),
}

impl<I> From<io::Error> for Error<I> {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl<I> From<num::ParseIntError> for Error<I> {
    fn from(e: num::ParseIntError) -> Self {
        Self::Num(e)
    }
}

impl<I> From<net::AddrParseError> for Error<I> {
    fn from(e: net::AddrParseError) -> Self {
        Self::IpAddr(e)
    }
}

impl<I> From<chrono::format::ParseError> for Error<I> {
    fn from(e: chrono::format::ParseError) -> Self {
        Self::DateTime(e)
    }
}

impl<I> From<(I, nom::error::ErrorKind)> for Error<I> {
    fn from(e: (I, nom::error::ErrorKind)) -> Self {
        Self::Parse(e)
    }
}

impl<I> From<Box<dyn std::error::Error>> for Error<I> {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Self::Other(e)
    }
}

impl<I> fmt::Display for Error<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(ref e) => write!(f, "{}", e),
            Self::Num(ref e) => write!(f, "{}", e),
            Self::IpAddr(ref e) => write!(f, "{}", e),
            Self::DateTime(ref e) => write!(f, "{}", e),
            Self::Parse((_, kind)) => write!(f, "{:?}", kind),
            Self::Other(e) => write!(f, "{}", e),
        }
    }
}

impl<I> nom::error::ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Self::Parse((input, kind))
    }

    fn append(input: I, kind: nom::error::ErrorKind, _other: Self) -> Self {
        Self::Parse((input, kind))
    }
}

impl<I> Error<I> {
    pub fn make_nom_error(i: I, kind: nom::error::ErrorKind) -> nom::Err<Self> {
        return nom::Err::Error(Self::Parse((i, kind)));
    }
}
