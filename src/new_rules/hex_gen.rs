//! Totalistic hexagonal Generations rules.

use crate::{
    error::{ConvertRuleError, ParseRuleError},
    new_rules::hex::HexRule,
    traits::{ParseGenRule, PrintGenRule, TotalisticGen},
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
/// # Examples
///
/// ```
/// use ca_rules::new_rules::HexGenRule;
/// use ca_rules::traits::*;
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

impl TotalisticGen for HexGenRule {
    const NBHD_SIZE: usize = 7;

    const SUFFIX: Option<char> = Some('H');

    #[inline]
    fn from_data(data: FixedBitSet, gen: u32) -> Self {
        Self { data, gen }
    }

    #[inline]
    fn data(&self) -> &FixedBitSet {
        &self.data
    }

    #[inline]
    fn gen(&self) -> u32 {
        self.gen
    }
}

impl Default for HexGenRule {
    #[inline]
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(14),
            gen: 2,
        }
    }
}
impl FromStr for HexGenRule {
    type Err = ParseRuleError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl Display for HexGenRule {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.gen() != 2 {
            f.write_str(&self.to_string_sbg())
        } else {
            f.write_str(&self.to_string_bsg())
        }
    }
}

impl From<HexRule> for HexGenRule {
    #[inline]
    fn from(rule: HexRule) -> Self {
        Self {
            data: rule.data,
            gen: 2,
        }
    }
}

impl TryFrom<HexGenRule> for HexRule {
    type Error = ConvertRuleError;

    #[inline]
    fn try_from(rule: HexGenRule) -> Result<Self, Self::Error> {
        if rule.gen != 2 {
            Err(ConvertRuleError::GenGreaterThan2)
        } else {
            Ok(Self { data: rule.data })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::ParseRule;

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
    fn parse_rule_nongen() -> Result<(), ParseRuleError> {
        let rule = HexGenRule::parse_rule("B2/S34H")?;

        let b: Vec<u8> = rule.iter_b().collect();
        let s: Vec<u8> = rule.iter_s().collect();
        let gen = rule.gen();

        assert_eq!(b, vec![2]);
        assert_eq!(s, vec![3, 4]);
        assert_eq!(gen, 2);

        assert_eq!(rule.to_string_bsg(), "B2/S34H");
        assert_eq!(rule.to_string_sbg(), "34/2H");
        assert_eq!(rule.to_string_catagolue(), "b2s34h");

        let non_gen = HexRule::parse_rule("B2/S34H")?;
        assert_eq!(HexRule::try_from(rule).unwrap(), non_gen);

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
