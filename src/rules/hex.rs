//! Totalistic hexagonal rules.

use super::Gen;
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
/// use ca_rules::ParseHex;
///
/// #[derive(Debug, Eq, PartialEq)]
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
/// let life = Rule::parse_rule("B2/S34H").unwrap();
///
/// assert_eq!(
///     life,
///     Rule {
///         b: vec![2],
///         s: vec![3, 4],
///     }
/// )
/// ```
pub trait ParseHex {
    /// Construct the rule from `b` / `s` data.
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Hex { b, s } = Hex::parse_rule(input)?;
        Ok(Self::from_bs(b, s))
    }
}

/// A trait for parsing [totalistic hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood)
/// [Generations](http://www.conwaylife.com/wiki/Generations) rules.
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::ParseHexGen;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
///     gen: usize,
/// }
///
/// impl ParseHexGen for Rule {
///     fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self {
///         Rule { b, s, gen }
///     }
/// }
///
/// let life = Rule::parse_rule("g4b24s13h").unwrap();
///
/// assert_eq!(
///     life,
///     Rule {
///         b: vec![2, 4],
///         s: vec![1, 3],
///         gen: 4,
///     }
/// )
/// ```
pub trait ParseHexGen {
    /// Construct the rule from `b` / `s` data and the number of states.
    fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Gen {
            rule: Hex { b, s },
            gen,
        } = Hex::parse_rule_gen(input)?;
        Ok(Self::from_bsg(b, s, gen))
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

    struct GenRule;

    impl ParseHexGen for GenRule {
        fn from_bsg(_b: Vec<u8>, _s: Vec<u8>, _gen: usize) -> Self {
            GenRule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule("B3/S23H")?;
        Rule::parse_rule("B3S23H")?;
        Rule::parse_rule("b3s23h")?;
        Rule::parse_rule("23/3H")?;
        Rule::parse_rule("23/h")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() -> Result<(), ParseRuleError> {
        assert_eq!(
            Rule::parse_rule("B3/S23ha").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            Rule::parse_rule("B3H/S23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule("B3/S23").err(),
            Some(ParseRuleError::Missing('H'))
        );
        assert_eq!(
            Rule::parse_rule("B3/S27H").err(),
            Some(ParseRuleError::Missing('H'))
        );
        assert_eq!(
            Rule::parse_rule("233h").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }

    #[test]
    fn valid_rules_gen() -> Result<(), ParseRuleError> {
        GenRule::parse_rule("B3/S23/C3H")?;
        GenRule::parse_rule("B3S23G3H")?;
        GenRule::parse_rule("g3b3s23h")?;
        GenRule::parse_rule("B3/S23H")?;
        GenRule::parse_rule("23/3/3h")?;
        GenRule::parse_rule("23//3H")?;
        GenRule::parse_rule("23/3h")?;
        Ok(())
    }

    #[test]
    fn invalid_rules_gen() -> Result<(), ParseRuleError> {
        assert_eq!(
            GenRule::parse_rule("B3/S23").err(),
            Some(ParseRuleError::Missing('H'))
        );
        assert_eq!(
            GenRule::parse_rule("B3/S23/H").err(),
            Some(ParseRuleError::MissingNumber)
        );
        assert_eq!(
            GenRule::parse_rule("g1b3s23h").err(),
            Some(ParseRuleError::GenLessThan2)
        );
        assert_eq!(
            GenRule::parse_rule("2333h").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }
}
