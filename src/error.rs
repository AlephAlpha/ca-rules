//! Errors that can be returned when parsing rule strings.

use displaydoc::Display;
use thiserror::Error;

/// Errors that can be returned when parsing rule strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Error)]
pub enum ParseRuleError {
    /// Missing expected {0:?}
    Missing(char),
    /// Missing expected number
    MissingNumber,
    /// Unexpected {0:?}
    Unexpected(char),
    /// Extra unparsed junk at the end of the rule string
    ExtraJunk,
    /// Number of states less than 2 in Generations rule
    GenLessThan2,
    /// Not a MAP rule
    NotMapRule,
    /// Invalid Base64 encoding for MAP rule
    Base64Error,
    /// Invalid length for MAP rule
    InvalidLength,
    /// Generations number overflow for Generations rule
    GenOverflow,
}

/// Errors that can be returned when converting rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Error)]
pub enum ConvertRuleError {
    /// Generations number greater than 2 when converting to non-Generations rules
    GenGreaterThan2,
    /// Not a totalistic rule
    NotTotalistic,
}
