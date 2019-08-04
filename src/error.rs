use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseRuleError {
    Missing(char),
    Unexpected(char),
    ExtraJunk,
}

impl Display for ParseRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            ParseRuleError::Missing(c) => format!("Missing expected {:?}", c),
            ParseRuleError::Unexpected(c) => format!("Unexpected {:?}", c),
            ParseRuleError::ExtraJunk => String::from("Extra unparsed junk at end of rule string"),
        };
        write!(f, "{}", message)
    }
}

impl Error for ParseRuleError {}
