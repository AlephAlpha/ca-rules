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
}

impl Display for ParseRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            ParseRuleError::Missing(c) => format!("Missing expected {:?}", c),
            ParseRuleError::MissingNumber => "Missing expected number".to_string(),
            ParseRuleError::Unexpected(c) => format!("Unexpected {:?}", c),
            ParseRuleError::ExtraJunk => {
                "Extra unparsed junk at the end of the rule string".to_string()
            }
            ParseRuleError::GenLessThan2 => {
                "Number of states less than 2 in Generations rule".to_string()
            }
        };
        write!(f, "{}", message)
    }
}

impl Error for ParseRuleError {}
