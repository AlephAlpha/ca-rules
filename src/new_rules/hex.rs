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
    while let Some(d) = chars.peek().and_then(|c| c.to_digit(7)) {
        chars.next();
        match bs {
            B => data.insert(d as usize),
            S => data.insert((d as usize) + 7),
        }
    }
}

/// [Totalistic hexagonal rules](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood).
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::HexRule;
///
/// let rule: HexRule = "B2/S34H".parse().unwrap();
///
/// let b: Vec<u8> = rule.iter_b().collect();
/// let s: Vec<u8> = rule.iter_s().collect();
///
/// assert_eq!(b, vec![2]);
/// assert_eq!(s, vec![3, 4]);
///
/// assert_eq!(rule.to_string(), "B2/S34H");
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexRule {
    pub(crate) data: FixedBitSet,
}

impl HexRule {
    /// Whether the rule contains this `b` data.
    pub fn contains_b(&self, b: u8) -> bool {
        self.data.contains(b as usize)
    }

    /// Whether the rule contains this `s` data.
    pub fn contains_s(&self, s: u8) -> bool {
        self.data.contains(s as usize + 7)
    }

    /// An iterator over the `b` data of the rule.
    pub fn iter_b(&self) -> impl Iterator<Item = u8> + '_ {
        self.data
            .ones()
            .filter_map(|bit| (bit < 7).then(|| bit as u8))
    }

    /// An iterator over the `s` data of the rule.
    pub fn iter_s(&self) -> impl Iterator<Item = u8> + '_ {
        self.data
            .ones()
            .filter_map(|bit| (bit >= 7).then(|| (bit - 7) as u8))
    }
}

impl Default for HexRule {
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(14),
        }
    }
}

impl ParseRule for HexRule {
    const DATA_SIZE: usize = 14;
    const SUFFIX: Option<char> = Some('H');

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

impl FromStr for HexRule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl PrintRule for HexRule {
    const SUFFIX: Option<char> = Some('H');

    fn write_bs(&self, string: &mut String, bs: Bs) {
        match bs {
            B => {
                for b in self.iter_b() {
                    string.push(char::from_digit(b as u32, 7).unwrap());
                }
            }
            S => {
                for s in self.iter_s() {
                    string.push(char::from_digit(s as u32, 7).unwrap());
                }
            }
        }
    }
}

impl Display for HexRule {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.to_string_bs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rule() -> Result<(), ParseRuleError> {
        let rule = HexRule::parse_rule("B2/S34H")?;

        let b: Vec<u8> = rule.iter_b().collect();
        let s: Vec<u8> = rule.iter_s().collect();

        assert_eq!(b, vec![2]);
        assert_eq!(s, vec![3, 4]);

        assert_eq!(rule.to_string_bs(), "B2/S34H");
        assert_eq!(rule.to_string_sb(), "34/2H");
        assert_eq!(rule.to_string_catagolue(), "b2s34h");
        Ok(())
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        HexRule::parse_rule("B3/S23H")?;
        HexRule::parse_rule("B3S23H")?;
        HexRule::parse_rule("b3s23h")?;
        HexRule::parse_rule("23/3H")?;
        HexRule::parse_rule("23/h")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() {
        assert_eq!(
            HexRule::parse_rule("B3/S23ha").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            HexRule::parse_rule("B3H/S23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            HexRule::parse_rule("B3/S23").err(),
            Some(ParseRuleError::Missing('H'))
        );
        assert_eq!(
            HexRule::parse_rule("B3/S27H").err(),
            Some(ParseRuleError::Missing('H'))
        );
        assert_eq!(
            HexRule::parse_rule("233h").err(),
            Some(ParseRuleError::Missing('/'))
        );
    }
}
