//! Neighborhood types.

use crate::error::ParseRuleError;
pub use hex::Hex;
pub use isohex::Isohex;
pub use isotropic::Isotropic;
pub use lifelike::Lifelike;
pub use neumann::Neumann;
use std::iter::Peekable;

/// A trait for neighborhood types.
pub trait Neighborhood {
    /// A suffix char at the end of the rule string that denotes the neighborhood type,
    /// e.g., `H` in the hexagonal rule `B2/S34H`.
    ///
    /// It is `None` if such a suffix is not needed.
    const SUFFIX: Option<char>;

    /// Parsing `b` or `s` data, i,e., the `3` or `23` part of the rule string `B3/S23`.
    fn parse_bs<I>(chars: &mut Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
    where
        I: Iterator<Item = char>;
}

/// A macro to define a function to parse `b` or `s` data for totalistic rules.
macro_rules! parse_bs_totalistic {
    ($n: expr) => {
        fn parse_bs<I>(chars: &mut Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
        where
            I: Iterator<Item = char>,
        {
            let mut bs = Vec::new();

            while let Some(&c) = chars.peek() {
                match c {
                    c if c.is_digit($n + 1) => {
                        chars.next();
                        bs.push(c.to_digit($n + 1).unwrap() as u8);
                    }
                    _ => return Ok(bs),
                }
            }
            Ok(bs)
        }
    };
}

/// A macro to define a function to parse `b` or `s` data for isotropic non-totalistic rules.
macro_rules! parse_bs_nontotalistic {
    { $($count: expr => { $($key: expr => $value: expr),* $(,)? }),*  $(,)? } => {
        fn parse_bs<I>(chars: &mut Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
        where
            I: Iterator<Item = char>,
        {
            let mut bs = Vec::new();

            while let Some(&c) = chars.peek() {
                match c {
                    $(
                        $count => {
                            chars.next();
                            let all_keys = vec![$($key),*];
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
                                        $key => bs.extend_from_slice(&($value)),
                                    )*
                                    _ => unreachable!(),
                                }
                            }
                        }
                    ),*
                    _ => return Ok(bs),
                }
            }
            Ok(bs)
        }
    };
}

mod hex;
mod isohex;
mod isotropic;
mod lifelike;
mod neumann;
