//! Totalistic life-like rules.

use super::Gen;
use crate::ParseRuleError;

rule_struct!(Life);

impl Life {
    parse_bs!(8);
    parse_rule!();
}

/// A trait for parsing [totalistic life-like rules](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton).
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::ParseLife;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseLife for Rule {
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule("B3/S23").unwrap();
///
/// assert_eq!(
///     life,
///     Rule {
///         b: vec![3],
///         s: vec![2, 3],
///     }
/// )
/// ```
pub trait ParseLife {
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Life { b, s } = Life::parse_rule(input)?;
        Ok(Self::from_bs(b, s))
    }
}

/// A trait for parsing [totalistic life-like](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton)
/// [Generations](http://www.conwaylife.com/wiki/Generations) rules.
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::ParseLifeGen;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
///     gen: usize,
/// }
///
/// impl ParseLifeGen for Rule {
///     fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self {
///         Rule { b, s, gen }
///     }
/// }
///
/// let life = Rule::parse_rule("3457/357/5").unwrap();
///
/// assert_eq!(
///     life,
///     Rule {
///         b: vec![3, 5, 7],
///         s: vec![3, 4, 5, 7],
///         gen: 5,
///     }
/// )
/// ```
pub trait ParseLifeGen {
    fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self;

    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Gen {
            rule: Life { b, s },
            gen,
        } = Life::parse_rule_gen(input)?;
        Ok(Self::from_bsg(b, s, gen))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rule;

    impl ParseLife for Rule {
        fn from_bs(_b: Vec<u8>, _s: Vec<u8>) -> Self {
            Rule
        }
    }

    struct GenRule;

    impl ParseLifeGen for GenRule {
        fn from_bsg(_b: Vec<u8>, _s: Vec<u8>, _gen: usize) -> Self {
            GenRule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule("B3/S23")?;
        Rule::parse_rule("B3S23")?;
        Rule::parse_rule("b3s23")?;
        Rule::parse_rule("23/3")?;
        Rule::parse_rule("23/")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() -> Result<(), ParseRuleError> {
        assert_eq!(
            Rule::parse_rule("B3/S23h").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            Rule::parse_rule("B3/23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule("B2e3-anq/S12-a3").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule("233").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }

    #[test]
    fn valid_rules_gen() -> Result<(), ParseRuleError> {
        GenRule::parse_rule("B3/S23/C3")?;
        GenRule::parse_rule("B3S23G3")?;
        GenRule::parse_rule("g3b3s23")?;
        GenRule::parse_rule("B3/S23")?;
        GenRule::parse_rule("23/3/3")?;
        GenRule::parse_rule("23//3")?;
        GenRule::parse_rule("23/3")?;
        Ok(())
    }

    #[test]
    fn invalid_rules_gen() -> Result<(), ParseRuleError> {
        assert_eq!(
            GenRule::parse_rule("B3/S23h").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            GenRule::parse_rule("B3/S23/").err(),
            Some(ParseRuleError::MissingNumber)
        );
        assert_eq!(
            GenRule::parse_rule("g1b3s23").err(),
            Some(ParseRuleError::GenLessThan2)
        );
        assert_eq!(
            GenRule::parse_rule("2333").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }
}
