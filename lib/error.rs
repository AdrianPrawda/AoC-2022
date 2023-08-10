use core::fmt;
use std::{error::{self, Error}, str::FromStr};

// Master Error / InternalSystemError

#[derive(Debug)]
pub enum InternalSystemError {
    CLIArgs(CLIArgsError),
    SolutionNotImplemented(SolutionNotImplementedError),
    SolutionErr(SolutionError),
}

impl fmt::Display for InternalSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalSystemError::CLIArgs(ref e) => e.fmt(f),
            InternalSystemError::SolutionNotImplemented(ref e) => e.fmt(f),
            InternalSystemError::SolutionErr(ref e) => e.fmt(f),
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

// SolutionError

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

impl error::Error for SolutionError { }