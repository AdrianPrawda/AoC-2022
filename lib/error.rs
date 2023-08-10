use core::fmt;
use std::{error::{self, Error}, str::FromStr, num::ParseIntError};

// Master Error / InternalSystemError

#[derive(Debug)]
pub enum InternalSystemError {
    CLIArgs(CLIArgsError),
    SolutionNotImplemented(SolutionNotImplementedError),
    SolutionErr(SolutionError),
    Conversion(ConversionError),
}

impl fmt::Display for InternalSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalSystemError::CLIArgs(ref e) => e.fmt(f),
            InternalSystemError::SolutionNotImplemented(ref e) => e.fmt(f),
            InternalSystemError::SolutionErr(ref e) => e.fmt(f),
            InternalSystemError::Conversion(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for InternalSystemError { }

impl From<CLIArgsError> for InternalSystemError {
    fn from(value: CLIArgsError) -> Self {
        InternalSystemError::CLIArgs(value)
    }
}

impl From<SolutionNotImplementedError> for InternalSystemError {
    fn from(value: SolutionNotImplementedError) -> Self {
        InternalSystemError::SolutionNotImplemented(value)
    }
}

impl From<SolutionError> for InternalSystemError {
    fn from(value: SolutionError) -> Self {
        InternalSystemError::SolutionErr(value)
    }
}

impl From<ConversionError> for InternalSystemError {
    fn from(value: ConversionError) -> Self {
        InternalSystemError::Conversion(value)
    }
}

impl From<std::io::Error> for InternalSystemError {
    fn from(value: std::io::Error) -> Self {
        InternalSystemError::SolutionErr(SolutionError::new(None, Some(Box::new(value)) ))
    }
}

impl From<ParseIntError> for InternalSystemError {
    fn from(value: ParseIntError) -> Self {
        InternalSystemError::SolutionErr(SolutionError::new(None,Some(Box::new(value))))
    }
}

// CLIArgsError

#[derive(Debug)]
pub struct CLIArgsError {
    mssg: String
}

impl CLIArgsError {
    pub fn new(mssg: &str) -> Self {
        CLIArgsError { mssg: String::from_str(mssg).unwrap() }
    }
}

impl fmt::Display for CLIArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mssg)
    }
}

impl error::Error for CLIArgsError { }

// SolutionNotImplementedError

#[derive(Debug)]
pub struct SolutionNotImplementedError {
    num: u32
}

impl SolutionNotImplementedError {
    pub fn new(solution_num: u32) -> Self {
       SolutionNotImplementedError { num: solution_num }
    }
}

impl fmt::Display for SolutionNotImplementedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Solution {} not implemented!", self.num)
    }
}

impl error::Error for SolutionNotImplementedError { }

// ConversionError

#[derive(Debug)]
pub struct ConversionError {
    from: String,
    into: String,
}

impl ConversionError {
    pub fn new(from: &str, into: &str) -> Self {
        ConversionError { 
            from: String::from_str(from).unwrap(), 
            into: String::from_str(into).unwrap()
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Couldn't convert from {} into {}", self.from, self.into)
    }
}

impl error::Error for ConversionError { }

// SolutionError

#[macro_export]
macro_rules! solution_err_mssg {
    ($err_mssg: literal) => {
        Err(SolutionError::new( Some(String::from_str($err_mssg).unwrap()), None ).into())
    };
}

#[derive(Debug)]
pub struct SolutionError {
    mssg: Option<String>,
    err: Option<Box<dyn Error>>,
}

impl SolutionError {
    pub fn new(mssg: Option<String>, err: Option<Box<dyn Error>>) -> Self {
        SolutionError { mssg, err }
    }
}

impl fmt::Display for SolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mssg.clone() {
            Some(mssg) => write!(f, "{}", &mssg),
            None => match self.err {
                Some(ref e) => e.fmt(f),
                None => write!(f, ""),
            },
        }
    }
}

impl error::Error for SolutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.err.as_deref()
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}