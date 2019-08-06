use crate::{error::ParseRuleError, neighborhood::Neighborhood};
use std::iter::Peekable;

/// A trait for rules of the form [`Bxx/Sxx`](http://www.conwaylife.com/wiki/Rulestring).
///
/// # Example:
/// ```
/// use ca_rules::{neighborhood, ParseBSRules};
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseBSRules for Rule {
///     type Neighborhood = neighborhood::Lifelike;
///
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule(&"B3/S23").unwrap();
/// assert_eq!(
///     life,
///     Rule {
///         b: vec![3],
///         s: vec![2, 3],
///     }
/// )
/// ```
pub trait ParseBSRules {
    /// The neighborhood type of the rule.
    type Neighborhood: Neighborhood;

    /// Construct the rule from `b` and `s` data.
    ///
    /// Please see the documents of different neighborhood types for the definition
    /// of `b` and `s` data.
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let mut chars = input.chars().peekable();
        let (b, s);

        match chars.peek() {
            Some('B') | Some('b') => {
                // Rule strings using B/S notation
                chars.next();
                b = Self::Neighborhood::parse_bs(&mut chars)?;
                match chars.peek() {
                    Some('/') => {
                        chars.next();
                    }
                    Some(_) => (),
                    None => return Err(ParseRuleError::Missing('/')),
                }
                match chars.next() {
                    Some('S') | Some('s') => (),
                    _ => return Err(ParseRuleError::Missing('S')),
                }
                s = Self::Neighborhood::parse_bs(&mut chars)?;
            }
            _ => {
                // Rule strings using S/B notation
                s = Self::Neighborhood::parse_bs(&mut chars)?;
                match chars.next() {
                    Some('/') => (),
                    _ => return Err(ParseRuleError::Missing('/')),
                }
                b = Self::Neighborhood::parse_bs(&mut chars)?;
            }
        }

        // Suffix
        if let Some(s) = Self::Neighborhood::SUFFIX {
            if let Some(c) = chars.next() {
                if s.to_lowercase().chain(s.to_uppercase()).all(|s| s != c) {
                    return Err(ParseRuleError::Missing(s));
                }
            } else {
                return Err(ParseRuleError::Missing(s));
            }
        }

        match chars.next() {
            None => Ok(Self::from_bs(b, s)),
            _ => Err(ParseRuleError::ExtraJunk),
        }
    }
}

/// A trait for [Generations](http://www.conwaylife.com/wiki/Generations) rules.
///
/// # Example:
/// ```
/// use ca_rules::{neighborhood, ParseGenerations};
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
///     n: usize,
/// }
///
/// impl ParseGenerations for Rule {
///     type Neighborhood = neighborhood::Lifelike;
///
///     fn from_bsc(b: Vec<u8>, s: Vec<u8>, n: usize) -> Self {
///         Rule { b, s, n }
///     }
/// }
///
/// let life = Rule::parse_rule(&"3457/357/5").unwrap();
/// assert_eq!(
///     life,
///     Rule {
///         b: vec![3, 5, 7],
///         s: vec![3, 4, 5, 7],
///         n: 5,
///     }
/// )
/// ```
pub trait ParseGenerations {
    /// The neighborhood type of the rule.
    type Neighborhood: Neighborhood;

    /// Construct the rule from `b` and `s` data, and the number of states, `n`.
    ///
    /// Please see the documents of different neighborhood types for the definition
    /// of `b` and `s` data.
    fn from_bsc(b: Vec<u8>, s: Vec<u8>, n: usize) -> Self;

    /// The parser.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let mut chars = input.chars().peekable();
        let (b, s);
        let mut n = 2;

        match chars.peek() {
            // Rule strings using B/S/C notation
            Some('B') | Some('b') => {
                chars.next();
                b = Self::Neighborhood::parse_bs(&mut chars)?;
                match chars.peek() {
                    Some('/') => {
                        chars.next();
                    }
                    Some(_) => (),
                    None => return Err(ParseRuleError::Missing('/')),
                }
                match chars.next() {
                    Some('S') | Some('s') => (),
                    _ => return Err(ParseRuleError::Missing('S')),
                }
                s = Self::Neighborhood::parse_bs(&mut chars)?;
                match chars.peek() {
                    Some('/') => {
                        chars.next();
                        match chars.peek() {
                            Some('C') | Some('c') | Some('G') | Some('g') => {
                                chars.next();
                            }
                            _ => (),
                        }
                        n = parse_num(&mut chars)?;
                    }
                    Some('C') | Some('c') | Some('G') | Some('g') => {
                        chars.next();
                        n = parse_num(&mut chars)?;
                    }
                    _ => (),
                }
            }

            // Rule strings using C/B/S notation
            Some('C') | Some('c') | Some('G') | Some('g') => {
                chars.next();
                n = parse_num(&mut chars)?;
                match chars.peek() {
                    Some('/') => {
                        chars.next();
                    }
                    Some(_) => (),
                    None => return Err(ParseRuleError::Missing('/')),
                }
                match chars.next() {
                    Some('B') | Some('b') => (),
                    _ => return Err(ParseRuleError::Missing('B')),
                }
                b = Self::Neighborhood::parse_bs(&mut chars)?;
                match chars.peek() {
                    Some('/') => {
                        chars.next();
                    }
                    Some(_) => (),
                    None => return Err(ParseRuleError::Missing('/')),
                }
                match chars.next() {
                    Some('S') | Some('s') => (),
                    _ => return Err(ParseRuleError::Missing('S')),
                }
                s = Self::Neighborhood::parse_bs(&mut chars)?;
            }

            // Rule strings using S/B/G notation
            _ => {
                s = Self::Neighborhood::parse_bs(&mut chars)?;
                match chars.next() {
                    Some('/') => (),
                    _ => return Err(ParseRuleError::Missing('/')),
                }
                b = Self::Neighborhood::parse_bs(&mut chars)?;
                match chars.peek() {
                    Some('/') => {
                        chars.next();
                        n = parse_num(&mut chars)?;
                    }
                    _ => (),
                }
            }
        }

        // Suffix
        if let Some(s) = Self::Neighborhood::SUFFIX {
            if let Some(c) = chars.next() {
                if s.to_lowercase().chain(s.to_uppercase()).all(|s| s != c) {
                    return Err(ParseRuleError::Missing(s));
                }
            } else {
                return Err(ParseRuleError::Missing(s));
            }
        }

        if n < 2 {
            Err(ParseRuleError::GenLessThan2)
        } else {
            match chars.next() {
                None => Ok(Self::from_bsc(b, s, n)),
                _ => Err(ParseRuleError::ExtraJunk),
            }
        }
    }
}

fn parse_num<I>(chars: &mut Peekable<I>) -> Result<usize, ParseRuleError>
where
    I: Iterator<Item = char>,
{
    let mut n = 0;
    if chars.peek().is_none() {
        return Err(ParseRuleError::MissingNumber);
    }
    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_digit(10) => {
                chars.next();
                n *= 10;
                n += c.to_digit(10).unwrap() as usize;
            }
            _ => return Ok(n),
        }
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{neighborhood::Lifelike, ParseGenerations};

    struct Rule;

    impl ParseGenerations for Rule {
        type Neighborhood = Lifelike;

        fn from_bsc(_b: Vec<u8>, _s: Vec<u8>, _n: usize) -> Self {
            Rule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule(&"B3/S23/C3")?;
        Rule::parse_rule(&"B3S23G3")?;
        Rule::parse_rule(&"g3b3s23")?;
        Rule::parse_rule(&"B3/S23")?;
        Rule::parse_rule(&"23/3/3")?;
        Rule::parse_rule(&"23//3")?;
        // Rule::parse_rule(&"23/3")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() -> Result<(), ParseRuleError> {
        assert_eq!(
            Rule::parse_rule(&"B3/S23h").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            Rule::parse_rule(&"B3/S23/").err(),
            Some(ParseRuleError::MissingNumber)
        );
        assert_eq!(
            Rule::parse_rule(&"g1b3s23").err(),
            Some(ParseRuleError::GenLessThan2)
        );
        assert_eq!(
            Rule::parse_rule(&"2333").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }
}
