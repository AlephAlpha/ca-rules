//! Errors that can be returned when parsing rule strings.

// use std::fmt::{self, Display, Formatter};
use thiserror::Error;

/// Errors that can be returned when parsing rule strings.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ParseRuleError {
    #[error("Missing expected {0:?}")]
    Missing(char),
    #[error("Missing expected number")]
    MissingNumber,
    #[error("Unexpected {0:?}")]
    Unexpected(char),
    #[error("Extra unparsed junk at the end of the rule string")]
    ExtraJunk,
    #[error("Number of states less than 2 in Generations rule")]
    GenLessThan2,
    #[error("Not a MAP rule")]
    NotMapRule,
    #[error("Invalid Base64 encoding for MAP rule")]
    Base64Error,
    #[error("Invalid length for MAP rule")]
    InvalidLength,
    #[error("Generations number overflow for Generations rule")]
    GenOverflow,
}
