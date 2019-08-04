use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum ParseRuleError {
    MissingNumber,
    MissingB,
    MissingS,
    MissingSlash,
    ExtraJunk,
}

impl Display for ParseRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            ParseRuleError::MissingNumber => &"Missing number in rule",
            ParseRuleError::MissingB => &"Expected B at start of rule",
            ParseRuleError::MissingS => &"Expected S after slash",
            ParseRuleError::MissingSlash => &"Missing expected slash between b and s",
            ParseRuleError::ExtraJunk => &"Extra unparsed junk at end of rule string",
        };
        write!(f, "{}", message)
    }
}

impl Error for ParseRuleError {}
