//! Totalistic hexagonal Generations rules.

use crate::{
    error::{ConverRuleError, ParseRuleError},
    new_rules::hex::{self, HexRule},
    traits::{ParseGenRule, PrintGenRule},
    util::Bs::{self, B, S},
};
use fixedbitset::FixedBitSet;
use std::{
    char,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// [Totalistic hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood)
/// [Generations](http://www.conwaylife.com/wiki/Generations) rules.
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::HexGenRule;
///
/// let rule: HexGenRule = "g4b24s13h".parse().unwrap();
///
/// let b: Vec<u8> = rule.iter_b().collect();
/// let s: Vec<u8> = rule.iter_s().collect();
/// let gen = rule.gen();
///
/// assert_eq!(b, vec![2, 4]);
/// assert_eq!(s, vec![1, 3]);
/// assert_eq!(gen, 4);
///
/// assert_eq!(rule.to_string(), "13/24/4H");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexGenRule {
    data: FixedBitSet,
    gen: u32,
}

impl HexGenRule {
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

    /// The generation number.
    pub fn gen(&self) -> u32 {
        self.gen
    }
}

impl Default for HexGenRule {
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(14),
            gen: 2,
        }
    }
}

impl ParseGenRule for HexGenRule {
    const DATA_SIZE: usize = 14;
    const SUFFIX: Option<char> = Some('H');

    fn read_bs<I>(data: &mut FixedBitSet, chars: &mut std::iter::Peekable<I>, bs: Bs)
    where
        I: Iterator<Item = char>,
    {
        hex::read_bs(data, chars, bs)
    }

    fn from_data(data: FixedBitSet, gen: u32) -> Self {
        Self { data, gen }
    }
}

impl FromStr for HexGenRule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl PrintGenRule for HexGenRule {
    const SUFFIX: Option<char> = Some('H');

    fn gen(&self) -> u32 {
        self.gen
    }

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

impl Display for HexGenRule {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.to_string_sbg())
    }
}

impl From<HexRule> for HexGenRule {
    fn from(rule: HexRule) -> Self {
        Self {
            data: rule.data,
            gen: 2,
        }
    }
}

impl TryFrom<HexGenRule> for HexRule {
    type Error = ConverRuleError;

    fn try_from(rule: HexGenRule) -> Result<Self, Self::Error> {
        if rule.gen != 2 {
            Err(ConverRuleError::GenGreaterThan2)
        } else {
            Ok(Self { data: rule.data })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rule() -> Result<(), ParseRuleError> {
        let rule = HexGenRule::parse_rule("13/24/4H")?;

        let b: Vec<u8> = rule.iter_b().collect();
        let s: Vec<u8> = rule.iter_s().collect();
        let gen = rule.gen();

        assert_eq!(b, vec![2, 4]);
        assert_eq!(s, vec![1, 3]);
        assert_eq!(gen, 4);

        assert_eq!(rule.to_string_bsg(), "B24/S13/G4H");
        assert_eq!(rule.to_string_sbg(), "13/24/4H");
        assert_eq!(rule.to_string_catagolue(), "g4b24s13h");
        Ok(())
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        HexGenRule::parse_rule("B3/S23/C3H")?;
        HexGenRule::parse_rule("B3S23G3H")?;
        HexGenRule::parse_rule("g3b3s23h")?;
        HexGenRule::parse_rule("B3/S23H")?;
        HexGenRule::parse_rule("23/3/3h")?;
        HexGenRule::parse_rule("23//3H")?;
        HexGenRule::parse_rule("23/3h")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() {
        assert_eq!(
            HexGenRule::parse_rule("B3/S23").err(),
            Some(ParseRuleError::Missing('H'))
        );
        assert_eq!(
            HexGenRule::parse_rule("B3/S23/H").err(),
            Some(ParseRuleError::MissingNumber)
        );
        assert_eq!(
            HexGenRule::parse_rule("g1b3s23h").err(),
            Some(ParseRuleError::GenLessThan2)
        );
        assert_eq!(
            HexGenRule::parse_rule("2333h").err(),
            Some(ParseRuleError::Missing('/'))
        );
    }
}
