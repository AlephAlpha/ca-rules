use super::Neighborhood;
use crate::error::ParseRuleError;
use std::iter::Peekable;

/// Neighborhood for [isotropic non-totalistic hexagonal rules](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood).
///
/// The `b` / `s` data of this neighborhood type consists of possible combinations of
/// the states of the 6 neighbors, represented by an 6-bit binary number,
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
/// use ca_rules::{neighborhood, ParseBSRules};
///
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseBSRules for Rule {
///     type Neighborhood = neighborhood::Isohex;
///
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule(&"B2/S34H").unwrap();
///
/// for b in 0..=63 {
///     assert_eq!(life.b.contains(&b), [2].contains(&b.count_ones()));
/// }
///
/// for s in 0..=63 {
///     assert_eq!(life.s.contains(&s), [3, 4].contains(&s.count_ones()));
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Isohex;

impl Neighborhood for Isohex {
    const SUFFIX: Option<char> = Some('H');

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
                                $key => bs.extend_from_slice(&( $value )),
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
                '1' => {
                    chars.next();
                    bs.extend_from_slice(&[0x01, 0x02, 0x04, 0x08, 0x10, 0x20]);
                }
                '2' => parse_keys! {
                    'o' => [0x03, 0x05, 0x0a, 0x14, 0x28, 0x30],
                    'm' => [0x06, 0x09, 0x11, 0x18, 0x22, 0x24],
                    'p' => [0x0c, 0x12, 0x21],
                },
                '3' => parse_keys! {
                    'o' => [0x07, 0x0b, 0x15, 0x2a, 0x34, 0x38],
                    'm' => [0x0d, 0x0e, 0x13, 0x16, 0x1a, 0x1c, 0x23, 0x25, 0x29, 0x2c, 0x31, 0x32],
                    'p' => [0x19, 0x26],
                },
                '4' => parse_keys! {
                    'o' => [0x0f, 0x17, 0x2b, 0x35, 0x3a, 0x3c],
                    'm' => [0x1b, 0x1d, 0x27, 0x2e, 0x36, 0x39],
                    'p' => [0x1e, 0x2d, 0x33],
                },
                '5' => {
                    chars.next();
                    bs.extend_from_slice(&[0x1f, 0x2f, 0x37, 0x3b, 0x3d, 0x3e]);
                }
                '6' => {
                    chars.next();
                    bs.push(0x3f);
                }
                '/' | 'S' | 's' | 'H' | 'h' => {
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
        type Neighborhood = Isohex;

        fn from_bs(_b: Vec<u8>, _s: Vec<u8>) -> Self {
            Rule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule(&"B3/S23H")?;
        Rule::parse_rule(&"b2os24mh")?;
        Rule::parse_rule(&"12m3o4m5/2o3-o4mH")?;
        Rule::parse_rule(&"B2o3p4-o5/S2-p3p45H")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() -> Result<(), ParseRuleError> {
        assert_eq!(
            Rule::parse_rule(&"B3/S23").err(),
            Some(ParseRuleError::Missing('H'))
        );
        assert_eq!(
            Rule::parse_rule(&"B2/o24mH").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule(&"b2o3-o4m12m3o4m5h").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule(&"B2o3p4-o5-/S2-p3p45H").err(),
            Some(ParseRuleError::Unexpected('-'))
        );
        Ok(())
    }
}
