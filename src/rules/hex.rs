use crate::ParseRuleError;

rule_struct!(Hex);

impl Hex {
    parse_bs!(6);
    parse_rule!('H');
}

/// A trait for parsing [totalistic hexagonal rules](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood).
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::rules::ParseHex;
///
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseHex for Rule {
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
pub trait ParseHex {
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Hex { b, s } = Hex::parse_rule(input)?;
        Ok(Self::from_bs(b, s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rule;

    impl ParseHex for Rule {
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
            Some(ParseRuleError::Missing('H'))
        );
        assert_eq!(
            Rule::parse_rule(&"233h").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }
}
