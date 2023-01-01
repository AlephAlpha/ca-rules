//! Non-totalistic life-like rules.

use super::{
    life::{ParseLife, ParseLifeGen},
    nthex::{ParseNtHex, ParseNtHexGen},
    ntneumann::{ParseNtNeumann, ParseNtNeumannGen},
    Gen,
};
use crate::ParseRuleError;

rule_struct!(NtLife);

impl NtLife {
    parse_bs! {
        '0' => {
            'c' => [0x00],
        },
        '1' => {
            'c' => [0x01, 0x04, 0x20, 0x80],
            'e' => [0x02, 0x08, 0x10, 0x40],
        },
        '2' => {
            'c' => [0x05, 0x21, 0x84, 0xa0],
            'e' => [0x0a, 0x12, 0x48, 0x50],
            'k' => [0x0c, 0x11, 0x22, 0x30, 0x41, 0x44, 0x82, 0x88],
            'a' => [0x03, 0x06, 0x09, 0x14, 0x28, 0x60, 0x90, 0xc0],
            'i' => [0x18, 0x42],
            'n' => [0x24, 0x81],
        },
        '3' => {
            'c' => [0x25, 0x85, 0xa1, 0xa4],
            'e' => [0x1a, 0x4a, 0x52, 0x58],
            'k' => [0x32, 0x4c, 0x51, 0x8a],
            'a' => [0x0b, 0x16, 0x68, 0xd0],
            'i' => [0x07, 0x29, 0x94, 0xe0],
            'n' => [0x0d, 0x15, 0x23, 0x61, 0x86, 0xa8, 0xb0, 0xc4],
            'y' => [0x31, 0x45, 0x8c, 0xa2],
            'q' => [0x26, 0x2c, 0x34, 0x64, 0x83, 0x89, 0x91, 0xc1],
            'j' => [0x0e, 0x13, 0x2a, 0x49, 0x54, 0x70, 0x92, 0xc8],
            'r' => [0x19, 0x1c, 0x38, 0x43, 0x46, 0x62, 0x98, 0xc2],
        },
        '4' => {
            'c' => [0xa5],
            'e' => [0x5a],
            'k' => [0x33, 0x4d, 0x55, 0x71, 0x8e, 0xaa, 0xb2, 0xcc],
            'a' => [0x0f, 0x17, 0x2b, 0x69, 0x96, 0xd4, 0xe8, 0xf0],
            'i' => [0x1d, 0x63, 0xb8, 0xc6],
            'n' => [0x27, 0x2d, 0x87, 0x95, 0xa9, 0xb4, 0xe1, 0xe4],
            'y' => [0x35, 0x65, 0x8d, 0xa3, 0xa6, 0xac, 0xb1, 0xc5],
            'q' => [0x36, 0x6c, 0x8b, 0xd1],
            'j' => [0x3a, 0x4e, 0x53, 0x59, 0x5c, 0x72, 0x9a, 0xca],
            'r' => [0x1b, 0x1e, 0x4b, 0x56, 0x6a, 0x78, 0xd2, 0xd8],
            't' => [0x39, 0x47, 0x9c, 0xe2],
            'w' => [0x2e, 0x74, 0x93, 0xc9],
            'z' => [0x3c, 0x66, 0x99, 0xc3],
        },
        '5' => {
            'c' => [0x5b, 0x5e, 0x7a, 0xda],
            'e' => [0xa7, 0xad, 0xb5, 0xe5],
            'k' => [0x75, 0xae, 0xb3, 0xcd],
            'a' => [0x2f, 0x97, 0xe9, 0xf4],
            'i' => [0x1f, 0x6b, 0xd6, 0xf8],
            'n' => [0x3b, 0x4f, 0x57, 0x79, 0x9e, 0xdc, 0xea, 0xf2],
            'y' => [0x5d, 0x73, 0xba, 0xce],
            'q' => [0x3e, 0x6e, 0x76, 0x7c, 0x9b, 0xcb, 0xd3, 0xd9],
            'j' => [0x37, 0x6d, 0x8f, 0xab, 0xb6, 0xd5, 0xec, 0xf1],
            'r' => [0x3d, 0x67, 0x9d, 0xb9, 0xbc, 0xc7, 0xe3, 0xe6],
        },
        '6' => {
            'c' => [0x5f, 0x7b, 0xde, 0xfa],
            'e' => [0xaf, 0xb7, 0xed, 0xf5],
            'k' => [0x77, 0x7d, 0xbb, 0xbe, 0xcf, 0xdd, 0xee, 0xf3],
            'a' => [0x3f, 0x6f, 0x9f, 0xd7, 0xeb, 0xf6, 0xf9, 0xfc],
            'i' => [0xbd, 0xe7],
            'n' => [0x7e, 0xdb],
        },
        '7' => {
            'c' => [0x7f, 0xdf, 0xfb, 0xfe],
            'e' => [0xbf, 0xef, 0xf7, 0xfd],
        },
        '8' => {
            'c' => [0xff],
        },
    }
    parse_rule!();
    parse_rule_map!(8);
}

impl_parser!(
    (ParseLife, ParseLifeGen) for NtLife,
    |i: u8| i.count_ones() as u8,
    0xff,
);

impl_parser!(
    (ParseNtHex, ParseNtHexGen) for NtLife,
    |i: u8| (i & 0xc0) >> 2 | (i & 0x18) >> 1 | (i & 0x03),
    0xff,
);

impl_parser!(
    (ParseNtNeumann, ParseNtNeumannGen) for NtLife,
    |i: u8| (i & 0x40) >> 3 | (i & 0x18) >> 2 | (i & 0x02) >> 1,
    0xff,
);

/// A trait for parsing [non-totalistic life-like rules](http://www.conwaylife.com/wiki/Non-totalistic_Life-like_cellular_automaton).
/// Both [isotropic](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton)
/// and [non-isotropic](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
/// rules are supported.
///
/// The `b` / `s` data of this type of rules consists of possible combinations of
/// states of the 8 neighbors, represented by an 8-bit binary number,
/// that cause a cell to be born / survive.
///
/// For example, the following neighborhood is represented by the number `42 = 0b00101010`:
/// ```plaintext
/// 0 0 1
/// 0 _ 1
/// 0 1 0
/// ```
///
/// # Examples
///
/// ```
/// use ca_rules::ParseNtLife;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseNtLife for Rule {
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule("B35y/S1e2-ci3-a5i").unwrap();
///
/// assert!(life.s.contains(&0x2a));
/// ```
pub trait ParseNtLife {
    /// Construct the rule from `b` / `s` data.
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let NtLife { b, s } = ParseLife::parse_rule(input)
            .or_else(|_| NtLife::parse_rule(input))
            .or_else(|e| ParseNtHex::parse_rule(input).map_err(|_| e))
            .or_else(|e| ParseNtNeumann::parse_rule(input).map_err(|_| e))
            .or_else(|e| {
                NtLife::parse_rule_map(input).map_err(|e_map| {
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

/// A trait for parsing [non-totalistic life-like](http://www.conwaylife.com/wiki/Non-totalistic_Life-like_cellular_automaton)
/// [Generations](http://www.conwaylife.com/wiki/Generations) rules.
/// Both [isotropic](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton)
/// and [non-isotropic](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
/// rules are supported.
///
/// The `b` / `s` data of this type of rules consists of possible combinations of
/// states of the 8 neighbors, represented by an 8-bit binary number,
/// that cause a cell to be born / survive.
///
/// For example, the following neighborhood is represented by the number `42 = 0b00101010`:
/// ```plaintext
/// 0 0 1
/// 0 _ 1
/// 0 1 0
/// ```
///
/// # Examples
///
/// ```
/// use ca_rules::ParseNtLifeGen;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
///     gen: usize,
/// }
///
/// impl ParseNtLifeGen for Rule {
///     fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self {
///         Rule { b, s, gen }
///     }
/// }
///
/// let life = Rule::parse_rule("g4b2c36k7s2ak34-a5-i").unwrap();
///
/// assert_eq!(life.gen, 4);
/// ```
pub trait ParseNtLifeGen {
    /// Construct the rule from `b` / `s` data and the number of states.
    fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Gen {
            rule: NtLife { b, s },
            gen,
        } = ParseLifeGen::parse_rule(input)
            .or_else(|_| NtLife::parse_rule_gen(input))
            .or_else(|e| ParseNtHexGen::parse_rule(input).map_err(|_| e))
            .or_else(|e| ParseNtNeumannGen::parse_rule(input).map_err(|_| e))
            .or_else(|e| {
                NtLife::parse_rule_gen_map(input).map_err(|e_map| {
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

    impl ParseNtLife for Rule {
        fn from_bs(_b: Vec<u8>, _s: Vec<u8>) -> Self {
            Rule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule("B3/S23")?;
        Rule::parse_rule("B3/S23V")?;
        Rule::parse_rule("B2e3-anq/S12-a3")?;
        Rule::parse_rule("B35y/S1e2-ci3-a5i")?;
        Rule::parse_rule("B2o3p4-o5/S2-p3p45H")?;
        Rule::parse_rule("MAPFgFoF2gXgH5oF4B+gH4A6A")?;
        Rule::parse_rule("B2i34cj6a7c8/S2-i3-a4ceit6in")?;
        Rule::parse_rule("1e2cik3ejqry4anrwz5a6k/2c3aenq4aijryz5cikqr6ac8")?;
        Rule::parse_rule("MAPARYXfhZofugWaH7oaIDogBZofuhogOiAaIDogIAAgAAWaH7oaIDogGiA6ICAAIAAaIDogIAAgACAAIAAAAAAAA")?;
        Rule::parse_rule("MAPARYXfhZofugWaH7oaIDogBZofuhogOiAaIDogIAAgAAWaH7oaIDogGiA6ICAAIAAaIDogIAAgACAAIAAAAAAAA==")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() {
        assert_eq!(
            Rule::parse_rule("12-a3/B2e3-anq").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            Rule::parse_rule("B35y/1e2-ci3-a5i").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule("B2i34cj6a7c82-i3-a4ceit6in").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule("B2c3aenq4aijryz5cikqrz6ac8/S1e2cik3ejqry4anrwz5a6k").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule("MAPARYXfhZofugWaH7oaIDogBZofuhogOiAaIDogIAAgAAWaH7oaIDogGiA6ICAAIAAaIDogIAAgACAAIA").err(),
            Some(ParseRuleError::InvalidLength)
        );
        assert_eq!(
            Rule::parse_rule("MAPARYXfhZofugWaH7oaIDogBZofuhogOiAaIDogIAAgAAWaH7oaIDogGiA6ICAAIAAaIDogIAAgACAAIAAAAAAAX").err(),
            Some(ParseRuleError::Base64Error)
        );
    }

    #[test]
    fn parse_life_as_ntlife() -> Result<(), ParseRuleError> {
        let rule: NtLife = ParseLife::parse_rule("B2/S23")?;
        for b in 0..=0xff {
            assert_eq!(rule.b.contains(&b), [2].contains(&b.count_ones()));
        }

        for s in 0..=0xff {
            assert_eq!(rule.s.contains(&s), [2, 3].contains(&s.count_ones()));
        }
        Ok(())
    }

    #[test]
    fn parse_hex_as_ntlife() -> Result<(), ParseRuleError> {
        let rule: NtLife = ParseNtHex::parse_rule("B2/S34H")?;
        for b in 0..=0xff {
            assert_eq!(
                rule.b.contains(&b),
                [2].contains(&(b & 0b1101_1011).count_ones())
            );
        }

        for s in 0..=0xff {
            assert_eq!(
                rule.s.contains(&s),
                [3, 4].contains(&(s & 0b1101_1011).count_ones())
            );
        }
        Ok(())
    }

    #[test]
    fn parse_neumann_as_ntlife() -> Result<(), ParseRuleError> {
        let rule: NtLife = ParseNtNeumann::parse_rule("B2/S013V")?;
        for b in 0..=0xff {
            assert_eq!(
                rule.b.contains(&b),
                [2].contains(&(b & 0b0101_1010).count_ones())
            );
        }

        for s in 0..=0xff {
            assert_eq!(
                rule.s.contains(&s),
                [0, 1, 3].contains(&(s & 0b0101_1010).count_ones())
            );
        }
        Ok(())
    }

    #[test]
    fn parse_map() -> Result<(), ParseRuleError> {
        let rule1: NtLife = NtLife::parse_rule("B3/S23")?;
        let rule2: NtLife = NtLife::parse_rule_map("MAPARYXfhZofugWaH7oaIDogBZofuhogOiAaIDogIAAgAAWaH7oaIDogGiA6ICAAIAAaIDogIAAgACAAIAAAAAAAA")?;
        assert_eq!(rule1, rule2);
        Ok(())
    }

    #[test]
    fn parse_gen_map() -> Result<(), ParseRuleError> {
        let rule1: Gen<NtLife> = NtLife::parse_rule_gen("3457/357/5")?;
        let rule2: Gen<NtLife> = NtLife::parse_rule_gen_map("MAPARYBFxZpF38WaRd/aZZ//hZpF39pln/+aZZ//pZp/ukWaRd/aZZ//mmWf/6Waf7paZZ//pZp/umWaf7paZbplg/5")?;
        assert_eq!(rule1, rule2);
        Ok(())
    }
}
