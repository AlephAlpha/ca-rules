//! Errors that can be returned when parsing rule strings.

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Errors that can be returned when parsing rule strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseRuleError {
    /// Missing an expected char
    Missing(char),
    /// Missing an expected number
    MissingNumber,
    /// An unexpected char
    Unexpected(char),
    /// Extra unparsed junk at the end of the rule string
    ExtraJunk,
    /// Number of states less than 2 in Generations rule
    GenLessThan2,
    /// Not a MAP rule
    NotMapRule,
    /// An error occurs when decoding the base64 string
    Base64Error,
    /// Invalid length for MAP rule
    InvalidLength,
    /// Generations number overflow for Generations rule
    GenOverflow,
}

impl Display for ParseRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseRuleError::Missing(c) => write!(f, "Missing expected {:?}", c),
            ParseRuleError::MissingNumber => write!(f, "Missing expected number"),
            ParseRuleError::Unexpected(c) => write!(f, "Unexpected {:?}", c),
            ParseRuleError::ExtraJunk => {
                write!(f, "Extra unparsed junk at the end of the rule string")
            }
            ParseRuleError::GenLessThan2 => {
                write!(f, "Number of states less than 2 in Generations rule")
            }
            ParseRuleError::NotMapRule => write!(f, "Not a MAP rule"),
            ParseRuleError::Base64Error => {
                write!(f, "An error occurs when decoding the base64 string")
            }
            ParseRuleError::InvalidLength => write!(f, "Invalid length for MAP rule"),
            ParseRuleError::GenOverflow => {
                write!(f, "Generations number overflow for Generations rule")
            }
        }
    }
}

impl Error for ParseRuleError {}
