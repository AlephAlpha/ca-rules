mod hex;
mod isohex;
mod isotropic;
mod lifelike;

use crate::error::ParseRuleError;
pub use hex::Hex;
pub use isohex::Isohex;
pub use isotropic::Isotropic;
pub use lifelike::Lifelike;
use std::iter::Peekable;

/// A trait for neighborhood types.
pub trait Neighborhood {
    /// A suffix char at the end of the rule string that denotes the neighborhood type,
    /// e.g., `H` in the hexagonal rule `B2/S34H`.
    ///
    /// It is `None` if such a suffix is not needed.
    const SUFFIX: Option<char>;

    /// Parsing `b` or `s` data, i,e., the `3` or `23` part of the rule string `B3/S23`.
    fn parse_bs<I>(chars: &mut Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
    where
        I: Iterator<Item = char>;
}
