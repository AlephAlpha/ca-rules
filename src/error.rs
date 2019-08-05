use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Errors that can be returned when parsing ruls strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseRuleError {
    /// Missing an expected char
    Missing(char),
    /// An unexpected char
    Unexpected(char),
    /// Extra unparsed junk at the end of the rule string
    ExtraJunk,
}

impl Display for ParseRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            ParseRuleError::Missing(c) => format!("Missing expected {:?}", c),
            ParseRuleError::Unexpected(c) => format!("Unexpected {:?}", c),
            ParseRuleError::ExtraJunk => {
                String::from("Extra unparsed junk at the end of the rule string")
            }
        };
        write!(f, "{}", message)
    }
}

impl Error for ParseRuleError {}
