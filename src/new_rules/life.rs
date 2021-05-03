//! Totalistic hexagonal rules.

use crate::{
    error::ParseRuleError,
    traits::{ParseRule, PrintRule},
    util::Bs::{self, B, S},
};
use fixedbitset::FixedBitSet;
use std::{
    char,
    fmt::{self, Display, Formatter},
    iter::Peekable,
    str::FromStr,
};

/// Reading `b`/`s` data.
pub(crate) fn read_bs<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>, bs: Bs)
where
    I: Iterator<Item = char>,
{
    while let Some(d) = chars.peek().and_then(|c| c.to_digit(9)) {
        chars.next();
        match bs {
            B => data.insert(d as usize),
            S => data.insert((d as usize) + 9),
        }
    }
}

/// [Totalistic life-like rules](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton).
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::LifeRule;
///
/// let rule: LifeRule = "B3/S23".parse().unwrap();
///
/// let b: Vec<u8> = rule.iter_b().collect();
/// let s: Vec<u8> = rule.iter_s().collect();
///
/// assert_eq!(b, vec![3]);
/// assert_eq!(s, vec![2, 3]);
///
/// assert_eq!(rule.to_string(), "B3/S23");
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LifeRule {
    pub(crate) data: FixedBitSet,
}

impl LifeRule {
    /// Whether the rule contains this `b` data.
    pub fn contains_b(&self, b: u8) -> bool {
        self.data.contains(b as usize)
    }

    /// Whether the rule contains this `s` data.
    pub fn contains_s(&self, s: u8) -> bool {
        self.data.contains(s as usize + 9)
    }

    /// An iterator over the `b` data of the rule.
    pub fn iter_b(&self) -> impl Iterator<Item = u8> + '_ {
        self.data
            .ones()
            .filter_map(|bit| (bit < 9).then(|| bit as u8))
    }

    /// An iterator over the `s` data of the rule.
    pub fn iter_s(&self) -> impl Iterator<Item = u8> + '_ {
        self.data
            .ones()
            .filter_map(|bit| (bit >= 9).then(|| (bit - 9) as u8))
    }
}

impl Default for LifeRule {
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(18),
        }
    }
}

impl ParseRule for LifeRule {
    const DATA_SIZE: usize = 18;
    const SUFFIX: Option<char> = None;

    fn read_bs<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>, bs: Bs)
    where
        I: Iterator<Item = char>,
    {
        read_bs(data, chars, bs)
    }

    fn from_data(data: FixedBitSet) -> Self {
        Self { data }
    }
}

impl FromStr for LifeRule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl PrintRule for LifeRule {
    const SUFFIX: Option<char> = None;

    fn write_bs(&self, string: &mut String, bs: Bs) {
        match bs {
            B => {
                for b in self.iter_b() {
                    string.push(char::from_digit(b as u32, 9).unwrap());
                }
            }
            S => {
                for s in self.iter_s() {
                    string.push(char::from_digit(s as u32, 9).unwrap());
                }
            }
        }
    }
}

impl Display for LifeRule {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.to_string_bs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rule() -> Result<(), ParseRuleError> {
        let rule = LifeRule::parse_rule("B3/S23")?;

        let b: Vec<u8> = rule.iter_b().collect();
        let s: Vec<u8> = rule.iter_s().collect();

        assert_eq!(b, vec![3]);
        assert_eq!(s, vec![2, 3]);

        assert_eq!(rule.to_string_bs(), "B3/S23");
        assert_eq!(rule.to_string_sb(), "23/3");
        assert_eq!(rule.to_string_catagolue(), "b3s23");
        Ok(())
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        LifeRule::parse_rule("B3/S23")?;
        LifeRule::parse_rule("B3S23")?;
        LifeRule::parse_rule("b3s23")?;
        LifeRule::parse_rule("23/3")?;
        LifeRule::parse_rule("23/")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() {
        assert_eq!(
            LifeRule::parse_rule("B3/S23h").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            LifeRule::parse_rule("B3/23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            LifeRule::parse_rule("B2e3-anq/S12-a3").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            LifeRule::parse_rule("233").err(),
            Some(ParseRuleError::Missing('/'))
        );
    }
}
