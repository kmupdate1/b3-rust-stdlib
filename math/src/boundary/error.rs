use core::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalError {
    InvalidRange,
}

impl fmt::Display for IntervalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IntervalError::InvalidRange => {
                write!(f, "lower threshold must not exceed upper threshold")
            }
        }
    }
}
