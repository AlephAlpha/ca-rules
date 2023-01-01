//! Non-totalistic rules with von Neumann neighborhood.

use super::{
    neumann::{ParseNeumann, ParseNeumannGen},
    Gen,
};
use crate::ParseRuleError;

rule_struct!(NtNeumann);

impl NtNeumann {
    parse_rule_map!(4);
}

impl_parser!(
    (ParseNeumann, ParseNeumannGen) for NtNeumann,
    |i: u8| i.count_ones() as u8,
    0x0f,
);

/// A trait for parsing non-totalistic rules with
/// [von Neumann neighborhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
/// Both [isotropic](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton)
/// and [non-isotropic](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
/// rules are supported.
///
/// The `b` / `s` data of this type of rules consists of possible combinations of
/// states of the 4 neighbors, represented by an 8-bit binary number,
/// that cause a cell to be born / survive.
///
/// For example, the following neighborhood is represented by the number `10 = 0b1010`:
/// ```plaintext
///   1
/// 0 _ 1
///   0
/// ```
///
/// # Examples
///
/// ```
/// use ca_rules::ParseNtNeumann;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseNtNeumann for Rule {
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule("MAPHmlphg").unwrap();
///
/// assert!(life.s.contains(&0x00));
/// ```
pub trait ParseNtNeumann {
    /// Construct the rule from `b` / `s` data.
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let NtNeumann { b, s } = ParseNeumann::parse_rule(input).or_else(|e| {
            NtNeumann::parse_rule_map(input).map_err(|e_map| {
                if e_map == ParseRuleError::NotMapRule {
                    e
                } else {
                    e_map
                }
            })
        })?;
        Ok(Self::from_bs(b, s))
    }
}

/// A trait for parsing non-totalistic [Generations](http://www.conwaylife.com/wiki/Generations)
/// rules with [von Neumann neighborhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
/// Both [isotropic](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton)
/// and [non-isotropic](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
/// rules are supported.
///
/// The `b` / `s` data of this type of rules consists of possible combinations of
/// states of the 4 neighbors, represented by an 8-bit binary number,
/// that cause a cell to be born / survive.
///
/// For example, the following neighborhood is represented by the number `10 = 0b1010`:
/// ```plaintext
///   1
/// 0 _ 1
///   0
/// ```
///
/// # Examples
///
/// ```
/// use ca_rules::ParseNtNeumannGen;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
///     gen: usize,
/// }
///
/// impl ParseNtNeumannGen for Rule {
///     fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self {
///         Rule { b, s, gen }
///     }
/// }
///
/// let life = Rule::parse_rule("MAPHmlphg/3").unwrap();
///
/// assert_eq!(life.gen, 3);
/// ```
pub trait ParseNtNeumannGen {
    /// Construct the rule from `b` / `s` data and the number of states.
    fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Gen {
            rule: NtNeumann { b, s },
            gen,
        } = ParseNeumannGen::parse_rule(input).or_else(|e| {
            NtNeumann::parse_rule_gen_map(input).map_err(|e_map| {
                if e_map == ParseRuleError::NotMapRule {
                    e
                } else {
                    e_map
                }
            })
        })?;
        Ok(Self::from_bsg(b, s, gen))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rule;

    impl ParseNtNeumann for Rule {
        fn from_bs(_b: Vec<u8>, _s: Vec<u8>) -> Self {
            Rule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule("B3/S23V")?;
        Rule::parse_rule("B3S23V")?;
        Rule::parse_rule("b3s23v")?;
        Rule::parse_rule("23/3V")?;
        Rule::parse_rule("23/v")?;
        Rule::parse_rule("MAPHmlphg")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() {
        assert_eq!(
            Rule::parse_rule("B3/S23va").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            Rule::parse_rule("B3V/S23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule("B3/S23").err(),
            Some(ParseRuleError::Missing('V'))
        );
        assert_eq!(
            Rule::parse_rule("B3/S25V").err(),
            Some(ParseRuleError::Missing('V'))
        );
        assert_eq!(
            Rule::parse_rule("233v").err(),
            Some(ParseRuleError::Missing('/'))
        );
        assert_eq!(
            Rule::parse_rule("MAPFgFoF2gXgH5oF4B+gH4A6A").err(),
            Some(ParseRuleError::InvalidLength)
        );
    }

    #[test]
    fn parse_neumann_as_ntneumann() -> Result<(), ParseRuleError> {
        let rule: NtNeumann = ParseNeumann::parse_rule("B2/S013V")?;
        for b in 0..=0x0f {
            assert_eq!(rule.b.contains(&b), [2].contains(&b.count_ones()));
        }

        for s in 0..=0x0f {
            assert_eq!(rule.s.contains(&s), [0, 1, 3].contains(&s.count_ones()));
        }
        Ok(())
    }

    #[test]
    fn parse_map() -> Result<(), ParseRuleError> {
        let rule1: NtNeumann = NtNeumann::parse_rule("B2/S013V")?;
        let rule2: NtNeumann = NtNeumann::parse_rule_map("MAPHmlphg")?;
        assert_eq!(rule1, rule2);
        Ok(())
    }
}
