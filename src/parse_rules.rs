use crate::error::ParseRuleError;
use std::{iter::Peekable, str::Chars};

pub trait Neighborhood {
    fn parse_bs(chars: &mut Peekable<Chars>) -> Result<Vec<u8>, ParseRuleError>;
}

pub trait ParseBSRules {
    type Neighborhood: Neighborhood;

    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let mut chars = input.chars().peekable();
        match chars.next() {
            Some('B') | Some('b') => (),
            _ => return Err(ParseRuleError::MissingB),
        }
        let b = Self::Neighborhood::parse_bs(&mut chars)?;
        match chars.peek() {
            Some('/') => {
                chars.next();
            }
            Some(_) => (),
            None => return Err(ParseRuleError::MissingSlash),
        }
        match chars.next() {
            Some('S') | Some('s') => (),
            _ => return Err(ParseRuleError::MissingS),
        }
        let s = Self::Neighborhood::parse_bs(&mut chars)?;
        match chars.next() {
            None => Ok(Self::from_bs(b, s)),
            _ => Err(ParseRuleError::ExtraJunk),
        }
    }
}
