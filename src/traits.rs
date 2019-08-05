use crate::error::ParseRuleError;
use std::iter::Peekable;

/// A trait for neighborhood types.
pub trait Neighborhood {
    /// A suffix char at the end of the rule string that denotes the neighborhood type,
    /// e.g., `H` in the hexagonal rule `B2/S34H`.
    ///
    /// It is `None` if such a suffix is not needed.
    const SUFFIX: Option<char>;

    /// Parsing `b` or `s` data, e.g., `3` or `23` in the rule string `B3/S23`.
    fn parse_bs<I>(chars: &mut Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
    where
        I: Iterator<Item = char>;
}

/// A trait for rules of the form `Bxx/Sxx`.
pub trait ParseBSRules {
    /// The neighborhood type of the rule.
    type Neighborhood: Neighborhood;

    /// Construct the rule from `b` and `s` data.
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let mut chars = input.chars().peekable();
        let (b, s);

        match chars.peek() {
            Some('B') | Some('b') => {
                // Rule strings using B/S notation
                chars.next();
                b = Self::Neighborhood::parse_bs(&mut chars)?;
                match chars.peek() {
                    Some('/') => {
                        chars.next();
                    }
                    Some(_) => (),
                    None => return Err(ParseRuleError::Missing('/')),
                }
                match chars.next() {
                    Some('S') | Some('s') => (),
                    _ => return Err(ParseRuleError::Missing('S')),
                }
                s = Self::Neighborhood::parse_bs(&mut chars)?;
            }
            _ => {
                // Rule strings using S/B notation
                s = Self::Neighborhood::parse_bs(&mut chars)?;
                match chars.next() {
                    Some('/') => (),
                    _ => return Err(ParseRuleError::Missing('/')),
                }
                b = Self::Neighborhood::parse_bs(&mut chars)?;
            }
        }

        // Suffix
        if let Some(s) = Self::Neighborhood::SUFFIX {
            if let Some(c) = chars.next() {
                if s.to_lowercase().chain(s.to_uppercase()).all(|s| s != c) {
                    return Err(ParseRuleError::Missing(s));
                }
            } else {
                return Err(ParseRuleError::Missing(s));
            }
        }

        match chars.next() {
            None => Ok(Self::from_bs(b, s)),
            _ => Err(ParseRuleError::ExtraJunk),
        }
    }
}
