use super::Neighborhood;
use crate::error::ParseRuleError;
use std::iter::Peekable;

/// The [von Neumann neighbourhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
///
/// The `b` / `s` data of this neighborhood type consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::{neighborhood, ParseBSRules};
///
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseBSRules for Rule {
///     type Neighborhood = neighborhood::Neumann;
///
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule(&"B2/S013V").unwrap();
///
/// for b in 0..=6 {
///     assert_eq!(life.b.contains(&b), [2].contains(&b));
/// }
///
/// for s in 0..=6 {
///     assert_eq!(life.s.contains(&s), [0, 1, 3].contains(&s));
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Neumann;

impl Neighborhood for Neumann {
    const SUFFIX: Option<char> = Some('V');

    fn parse_bs<I>(chars: &mut Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
    where
        I: Iterator<Item = char>,
    {
        let mut bs = Vec::new();

        while let Some(&c) = chars.peek() {
            match c {
                c if c.is_digit(5) => {
                    chars.next();
                    bs.push(c.to_digit(5).unwrap() as u8);
                }
                _ => return Ok(bs),
            }
        }
        Ok(bs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParseBSRules;

    struct Rule;

    impl ParseBSRules for Rule {
        type Neighborhood = Neumann;

        fn from_bs(_b: Vec<u8>, _s: Vec<u8>) -> Self {
            Rule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule(&"B3/S23V")?;
        Rule::parse_rule(&"B3S23V")?;
        Rule::parse_rule(&"b3s23v")?;
        Rule::parse_rule(&"23/3V")?;
        Rule::parse_rule(&"23/v")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() -> Result<(), ParseRuleError> {
        assert_eq!(
            Rule::parse_rule(&"B3/S23va").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            Rule::parse_rule(&"B3V/S23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule(&"B3/S23").err(),
            Some(ParseRuleError::Missing('V'))
        );
        assert_eq!(
            Rule::parse_rule(&"B3/S25V").err(),
            Some(ParseRuleError::Missing('V'))
        );
        assert_eq!(
            Rule::parse_rule(&"233v").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }
}
