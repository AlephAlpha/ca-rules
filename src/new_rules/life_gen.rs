//! Totalistic life-like Generations rules.

use crate::{
    error::{ConverRuleError, ParseRuleError},
    new_rules::life::{self, LifeRule},
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

/// [Totalistic life-like](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton)
/// [Generations](http://www.conwaylife.com/wiki/Generations) rules.
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::LifeGenRule;
///
/// let rule: LifeGenRule = "g5b357s3457".parse().unwrap();
///
/// let b: Vec<u8> = rule.iter_b().collect();
/// let s: Vec<u8> = rule.iter_s().collect();
/// let gen = rule.gen();
///
/// assert_eq!(b, vec![3, 5, 7]);
/// assert_eq!(s, vec![3, 4, 5, 7]);
/// assert_eq!(gen, 5);
///
/// assert_eq!(rule.to_string(), "3457/357/5");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LifeGenRule {
    data: FixedBitSet,
    gen: u32,
}

impl LifeGenRule {
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

    /// The generation number.
    pub fn gen(&self) -> u32 {
        self.gen
    }
}

impl Default for LifeGenRule {
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(18),
            gen: 2,
        }
    }
}

impl ParseGenRule for LifeGenRule {
    const DATA_SIZE: usize = 18;
    const SUFFIX: Option<char> = None;

    fn read_bs<I>(data: &mut FixedBitSet, chars: &mut std::iter::Peekable<I>, bs: Bs)
    where
        I: Iterator<Item = char>,
    {
        life::read_bs(data, chars, bs)
    }

    fn from_data(data: FixedBitSet, gen: u32) -> Self {
        Self { data, gen }
    }
}

impl FromStr for LifeGenRule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl PrintGenRule for LifeGenRule {
    const SUFFIX: Option<char> = None;

    fn gen(&self) -> u32 {
        self.gen
    }

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

impl Display for LifeGenRule {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.to_string_sbg())
    }
}

impl From<LifeRule> for LifeGenRule {
    fn from(rule: LifeRule) -> Self {
        Self {
            data: rule.data,
            gen: 2,
        }
    }
}

impl TryFrom<LifeGenRule> for LifeRule {
    type Error = ConverRuleError;

    fn try_from(rule: LifeGenRule) -> Result<Self, Self::Error> {
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
        let rule = LifeGenRule::parse_rule("3457/357/5")?;

        let b: Vec<u8> = rule.iter_b().collect();
        let s: Vec<u8> = rule.iter_s().collect();
        let gen = rule.gen();

        assert_eq!(b, vec![3, 5, 7]);
        assert_eq!(s, vec![3, 4, 5, 7]);
        assert_eq!(gen, 5);

        assert_eq!(rule.to_string_bsg(), "B357/S3457/G5");
        assert_eq!(rule.to_string_sbg(), "3457/357/5");
        assert_eq!(rule.to_string_catagolue(), "g5b357s3457");
        Ok(())
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        LifeGenRule::parse_rule("B3/S23/C3")?;
        LifeGenRule::parse_rule("B3S23G3")?;
        LifeGenRule::parse_rule("g3b3s23")?;
        LifeGenRule::parse_rule("B3/S23")?;
        LifeGenRule::parse_rule("23/3/3")?;
        LifeGenRule::parse_rule("23//3")?;
        LifeGenRule::parse_rule("23/3")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() {
        assert_eq!(
            LifeGenRule::parse_rule("B3/S23h").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            LifeGenRule::parse_rule("B3/S23/").err(),
            Some(ParseRuleError::MissingNumber)
        );
        assert_eq!(
            LifeGenRule::parse_rule("g1b3s23").err(),
            Some(ParseRuleError::GenLessThan2)
        );
        assert_eq!(
            LifeGenRule::parse_rule("2333").err(),
            Some(ParseRuleError::Missing('/'))
        );
        assert_eq!(
            LifeGenRule::parse_rule("23/3/18446744073709551617").err(),
            Some(ParseRuleError::GenOverflow)
        );
    }
}
