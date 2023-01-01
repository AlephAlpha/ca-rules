//! Non-totalistic hexagonal rules.

use super::{
    hex::{ParseHex, ParseHexGen},
    Gen,
};
use crate::ParseRuleError;

rule_struct!(NtHex);

impl NtHex {
    parse_bs! {
        '0' => {
            'o' => [0x00],
        },
        '1' => {
            'o' => [0x01, 0x02, 0x04, 0x08, 0x10, 0x20],
        },
        '2' => {
            'o' => [0x03, 0x05, 0x0a, 0x14, 0x28, 0x30],
            'm' => [0x06, 0x09, 0x11, 0x18, 0x22, 0x24],
            'p' => [0x0c, 0x12, 0x21],
        },
        '3' => {
            'o' => [0x07, 0x0b, 0x15, 0x2a, 0x34, 0x38],
            'm' => [0x0d, 0x0e, 0x13, 0x16, 0x1a, 0x1c, 0x23, 0x25, 0x29, 0x2c, 0x31, 0x32],
            'p' => [0x19, 0x26],
        },
        '4' => {
            'o' => [0x0f, 0x17, 0x2b, 0x35, 0x3a, 0x3c],
            'm' => [0x1b, 0x1d, 0x27, 0x2e, 0x36, 0x39],
            'p' => [0x1e, 0x2d, 0x33],
        },
        '5' => {
            'o' => [0x1f, 0x2f, 0x37, 0x3b, 0x3d, 0x3e],
        },
        '6' => {
            'o' => [0x3f],
        },
    }
    parse_rule!('H');
    parse_rule_map!(6);
}

impl_parser!(
    (ParseHex, ParseHexGen) for NtHex,
    |i: u8| i.count_ones() as u8,
    0x3f,
);

/// A trait for parsing [non-totalistic hexagonal rules](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood).
/// Both [isotropic](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton)
/// and [non-isotropic](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
/// rules are supported.
///
/// The `b` / `s` data of this type of rules consists of possible combinations of
/// states of the 6 neighbors, represented by an 8-bit binary number,
/// that cause a cell to be born / survive.
///
/// For example, the following neighborhood is represented by the number `42 = 0b101010`:
/// ```plaintext
///  1 0
/// 1 _ 0
///  1 0
/// ```
///
/// # Examples
///
/// ```
/// use ca_rules::ParseNtHex;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseNtHex for Rule {
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule("B2o3-o4m/S12m3o4m5H").unwrap();
///
/// assert!(life.s.contains(&0x2a));
/// ```
pub trait ParseNtHex {
    /// Construct the rule from `b` / `s` data.
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let NtHex { b, s } = ParseHex::parse_rule(input)
            .or_else(|_| NtHex::parse_rule(input))
            .or_else(|e| {
                NtHex::parse_rule_map(input).map_err(|e_map| {
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

/// A trait for parsing [non-totalistic hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood)
/// [Generations](http://www.conwaylife.com/wiki/Generations) rules.
/// Both [isotropic](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton)
/// and [non-isotropic](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
/// rules are supported.
///
/// The `b` / `s` data of this type of rules consists of possible combinations of
/// states of the 6 neighbors, represented by an 8-bit binary number,
/// that cause a cell to be born / survive.
///
/// For example, the following neighborhood is represented by the number `42 = 0b101010`:
/// ```plaintext
///  1 0
/// 1 _ 0
///  1 0
/// ```
///
/// # Examples
///
/// ```
/// use ca_rules::ParseNtHexGen;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
///     gen: usize,
/// }
///
/// impl ParseNtHexGen for Rule {
///     fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self {
///         Rule { b, s, gen }
///     }
/// }
///
/// let life = Rule::parse_rule("g3b2o6s2-o34m56h").unwrap();
///
/// assert_eq!(life.gen, 3);
/// ```
pub trait ParseNtHexGen {
    /// Construct the rule from `b` / `s` data and the number of states.
    fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Gen {
            rule: NtHex { b, s },
            gen,
        } = ParseHexGen::parse_rule(input)
            .or_else(|_| NtHex::parse_rule_gen(input))
            .or_else(|e| {
                NtHex::parse_rule_gen_map(input).map_err(|e_map| {
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

    impl ParseNtHex for Rule {
        fn from_bs(_b: Vec<u8>, _s: Vec<u8>) -> Self {
            Rule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule("B3/S23H")?;
        Rule::parse_rule("b2os24mh")?;
        Rule::parse_rule("12m3o4m5/2o3-o4mH")?;
        Rule::parse_rule("B2o3p4-o5/S2-p3p45H")?;
        Rule::parse_rule("MAPFgFoF2gXgH5oF4B+gH4A6A")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() {
        assert_eq!(
            Rule::parse_rule("B3/S23").err(),
            Some(ParseRuleError::Missing('H'))
        );
        assert_eq!(
            Rule::parse_rule("B2/o24mH").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule("b2o3-o4m12m3o4m5h").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule("B2o3p4-o5-p/S2-p3p45H").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule("MAPFgFoF2gXgH5oF4B+gH4A6AH").err(),
            Some(ParseRuleError::Base64Error)
        );
    }

    #[test]
    fn parse_hex_as_nthex() -> Result<(), ParseRuleError> {
        let rule: NtHex = ParseHex::parse_rule("B2/S34H")?;
        for b in 0..=0x3f {
            assert_eq!(rule.b.contains(&b), [2].contains(&b.count_ones()));
        }

        for s in 0..=0x3f {
            assert_eq!(rule.s.contains(&s), [3, 4].contains(&s.count_ones()));
        }
        Ok(())
    }

    #[test]
    fn parse_map() -> Result<(), ParseRuleError> {
        let rule1: NtHex = NtHex::parse_rule("B2/S34H")?;
        let rule2: NtHex = NtHex::parse_rule_map("MAPFgFoF2gXgH5oF4B+gH4A6A")?;
        assert_eq!(rule1, rule2);
        Ok(())
    }
}
