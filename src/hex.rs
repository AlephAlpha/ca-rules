use crate::{error::ParseRuleError, traits::Neighborhood};
use std::iter::Peekable;

/// Neighborhood for [totalistic hexagonal rules](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood).
///
/// # Examples
///
/// ```
/// use ca_rules::{Hex, ParseBSRules};
///
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseBSRules for Rule {
///     type Neighborhood = Hex;
///
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule(&"B2/S34H").unwrap();
///
/// for b in 0..=6 {
///     assert_eq!(life.b.contains(&b), [2].contains(&b));
/// }
///
/// for s in 0..=6 {
///     assert_eq!(life.s.contains(&s), [3, 4].contains(&s));
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Hex;

impl Neighborhood for Hex {
    const SUFFIX: Option<char> = Some('H');

    fn parse_bs<I>(chars: &mut Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
    where
        I: Iterator<Item = char>,
    {
        let mut bs = Vec::new();

        while let Some(&c) = chars.peek() {
            match c {
                c if c.is_digit(7) => {
                    chars.next();
                    bs.push(c.to_digit(7).unwrap() as u8);
                }
                '/' | 'S' | 's' | 'H' | 'h' => return Ok(bs),
                c => return Err(ParseRuleError::Unexpected(c)),
            }
        }
        Ok(bs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::ParseBSRules;

    struct Rule;

    impl ParseBSRules for Rule {
        type Neighborhood = Hex;

        fn from_bs(_b: Vec<u8>, _s: Vec<u8>) -> Self {
            Rule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule(&"B3/S23H")?;
        Rule::parse_rule(&"B3S23H")?;
        Rule::parse_rule(&"b3s23h")?;
        Rule::parse_rule(&"23/3H")?;
        Rule::parse_rule(&"23/h")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() -> Result<(), ParseRuleError> {
        assert_eq!(
            Rule::parse_rule(&"B3/S23ha").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            Rule::parse_rule(&"B3H/S23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule(&"B3/S23").err(),
            Some(ParseRuleError::Missing('H'))
        );
        assert_eq!(
            Rule::parse_rule(&"B3/S27H").err(),
            Some(ParseRuleError::Unexpected('7'))
        );
        assert_eq!(
            Rule::parse_rule(&"233h").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }
}
