use std::fmt;
use std::num;
use std::string::FromUtf8Error;

#[derive(Debug,PartialEq)]
pub enum RESPError {
    WrongType,
    FromUtf8,
    IncorrectLength(RESPLength),
    Unknown,
    ParseInt,
    OutOfBounds(usize),
}

pub type RESPResult<T> = Result<T, RESPError>;

pub type RESPLength = i32;

impl fmt::Display for RESPError {
        
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::FromUtf8 => write!(f,"Cannot convert from UTF-8"),
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
            RESPError::WrongType => write!(f,"Wrong prefix for RESP type"),
            RESPError::Unknown => write!(f,"Unknown format for the RESP String"),
            RESPError::IncorrectLength(length) => write!(f,"Incorrect Length {}",length),
            RESPError::ParseInt => write!(f,"Cannot parse the string into integer"),
        }
    }
}

impl From<FromUtf8Error> for RESPError{
    fn from(_err: FromUtf8Error) -> Self{
        Self::FromUtf8
    }
}

impl From<num::ParseIntError> for RESPError{
    fn from(_err: num::ParseIntError) -> Self{
        Self::ParseInt
    }
}