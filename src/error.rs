use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Errors that can be returned when parsing ruls strings.
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
    /// Number of states less than 2 in Generations rules
    GenLessThan2,
}

impl Display for ParseRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            ParseRuleError::Missing(c) => format!("Missing expected {:?}", c),
            ParseRuleError::MissingNumber => format!("Missing expected number"),
            ParseRuleError::Unexpected(c) => format!("Unexpected {:?}", c),
            ParseRuleError::ExtraJunk => {
                String::from("Extra unparsed junk at the end of the rule string")
            }
            ParseRuleError::GenLessThan2 => {
                String::from("Number of states less than 2 in Generations rules")
            }
        };
        write!(f, "{}", message)
    }
}

impl Error for ParseRuleError {}
