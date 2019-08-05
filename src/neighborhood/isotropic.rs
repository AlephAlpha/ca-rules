use super::Neighborhood;
use crate::error::ParseRuleError;
use std::iter::Peekable;

/// Neighborhood for [isotropic non-totalistic life-like rules](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton).
///
/// The `b` / `s` data of this neighborhood type consists of possible combinations of
/// the states of the 8 neighbors, represented by an 8-bit binary number,
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
/// use ca_rules::{neighborhood, ParseBSRules};
///
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseBSRules for Rule {
///     type Neighborhood = neighborhood::Isotropic;
///
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule(&"B3/S23").unwrap();
///
/// for b in 0..=255 {
///     assert_eq!(life.b.contains(&b), [3].contains(&b.count_ones()));
/// }
///
/// for s in 0..=255 {
///     assert_eq!(life.s.contains(&s), [2, 3].contains(&s.count_ones()));
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Isotropic;

impl Neighborhood for Isotropic {
    const SUFFIX: Option<char> = None;

    #[allow(clippy::cognitive_complexity)]
    fn parse_bs<I>(chars: &mut Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
    where
        I: Iterator<Item = char>,
    {
        let mut bs = Vec::new();

        macro_rules! parse_keys {
            ( $( $key: expr => $value: expr, )* ) => {
                {
                    chars.next();
                    let all_keys = vec![$( $key, )*];
                    let keys = match chars.peek() {
                        Some('-') => {
                            chars.next();
                            let mut keys = Vec::new();
                            while let Some(&c) = chars.peek() {
                                if all_keys.contains(&c) {
                                    chars.next();
                                    keys.push(c);
                                } else {
                                    break;
                                }
                            }
                            all_keys.into_iter().filter(|c| !keys.contains(c)).collect()
                        }
                        Some(c) if all_keys.contains(&c) => {
                            let mut keys = Vec::new();
                            while let Some(&c) = chars.peek() {
                                if all_keys.contains(&c) {
                                    chars.next();
                                    keys.push(c);
                                } else {
                                    break;
                                }
                            }
                            keys
                        }
                        Some(_) => {
                            all_keys
                        }
                        None => all_keys
                    };
                    for &c in keys.iter() {
                        match c {
                            $(
                                $key => {
                                    for &i in $value.iter() {
                                        bs.push(i);
                                    }
                                }
                            )*
                            _ => unreachable!(),
                        }
                    }
                }
            };
        }

        while let Some(&c) = chars.peek() {
            match c {
                '0' => {
                    chars.next();
                    bs.push(0x00);
                }
                '1' => parse_keys! {
                    'c' => [0x01, 0x04, 0x20, 0x80],
                    'e' => [0x02, 0x08, 0x10, 0x40],
                },
                '2' => parse_keys! {
                    'c' => [0x05, 0x21, 0x84, 0xa0],
                    'e' => [0x0a, 0x12, 0x48, 0x50],
                    'k' => [0x0c, 0x11, 0x22, 0x30, 0x41, 0x44, 0x82, 0x88],
                    'a' => [0x03, 0x06, 0x09, 0x14, 0x28, 0x60, 0x90, 0xc0],
                    'i' => [0x18, 0x42],
                    'n' => [0x24, 0x81],
                },
                '3' => parse_keys! {
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
                '4' => parse_keys! {
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
                '5' => parse_keys! {
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
                '6' => parse_keys! {
                    'c' => [0x5f, 0x7b, 0xde, 0xfa],
                    'e' => [0xaf, 0xb7, 0xed, 0xf5],
                    'k' => [0x77, 0x7d, 0xbb, 0xbe, 0xcf, 0xdd, 0xee, 0xf3],
                    'a' => [0x3f, 0x6f, 0x9f, 0xd7, 0xeb, 0xf6, 0xf9, 0xfc],
                    'i' => [0xbd, 0xe7],
                    'n' => [0x7e, 0xdb],
                },
                '7' => parse_keys! {
                    'c' => [0x7f, 0xdf, 0xfb, 0xfe],
                    'e' => [0xbf, 0xef, 0xf7, 0xfd],
                },
                '8' => {
                    chars.next();
                    bs.push(0x00);
                }
                '/' | 'S' | 's' => {
                    return Ok(bs);
                }
                c => return Err(ParseRuleError::Unexpected(c)),
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
        type Neighborhood = Isotropic;

        fn from_bs(_b: Vec<u8>, _s: Vec<u8>) -> Self {
            Rule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule(&"B3/S23")?;
        Rule::parse_rule(&"B2e3-anq/S12-a3")?;
        Rule::parse_rule(&"B35y/S1e2-ci3-a5i")?;
        Rule::parse_rule(&"B2i34cj6a7c8/S2-i3-a4ceit6in")?;
        Rule::parse_rule(&"1e2cik3ejqry4anrwz5a6k/2c3aenq4aijryz5cikqr6ac8")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() -> Result<(), ParseRuleError> {
        assert_eq!(
            Rule::parse_rule(&"12-a3/B2e3-anq").err(),
            Some(ParseRuleError::Unexpected('B'))
        );
        assert_eq!(
            Rule::parse_rule(&"B35y/1e2-ci3-a5i").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule(&"B3/S23h").err(),
            Some(ParseRuleError::Unexpected('h'))
        );
        assert_eq!(
            Rule::parse_rule(&"B2i34cj6a7c82-i3-a4ceit6in").err(),
            Some(ParseRuleError::Missing('/'))
        );
        assert_eq!(
            Rule::parse_rule(&"B2c3aenq4aijryz5cikqrz6ac8/S1e2cik3ejqry4anrwz5a6k").err(),
            Some(ParseRuleError::Unexpected('z'))
        );
        Ok(())
    }
}
