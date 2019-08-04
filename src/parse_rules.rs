use crate::error::ParseRuleError;
use std::iter::Peekable;

pub trait Neighborhood {
    const SUFFIX: Option<char>;

    fn parse_bs<I>(chars: &mut Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
    where
        I: Iterator<Item = char>;
}

pub trait ParseBSRules {
    type Neighborhood: Neighborhood;

    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let mut chars = input.chars().peekable();
        let (b, s);

        match chars.peek() {
            Some('B') | Some('b') => {
                // Rulestrings using B/S notation
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
                // Rulestrings using S/B notation
                s = Self::Neighborhood::parse_bs(&mut chars)?;
                match chars.next() {
                    Some('/') => (),
                    _ => return Err(ParseRuleError::Missing('/')),
                }
                b = Self::Neighborhood::parse_bs(&mut chars)?;
            }
        }

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
