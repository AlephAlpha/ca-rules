//! Totalistic life-like Generations rules.

use crate::{
    error::{ConvertRuleError, ParseRuleError},
    new_rules::life::LifeRule,
    traits::{ParseGenRule, PrintGenRule, TotalisticGen},
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
/// # Examples
///
/// ```
/// use ca_rules::new_rules::LifeGenRule;
/// use ca_rules::traits::*;
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

impl TotalisticGen for LifeGenRule {
    const NBHD_SIZE: usize = 9;

    const SUFFIX: Option<char> = None;

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

impl Default for LifeGenRule {
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(18),
            gen: 2,
        }
    }
}

impl FromStr for LifeGenRule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl Display for LifeGenRule {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.gen() != 2 {
            f.write_str(&self.to_string_sbg())
        } else {
            f.write_str(&self.to_string_bsg())
        }
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
    type Error = ConvertRuleError;

    fn try_from(rule: LifeGenRule) -> Result<Self, Self::Error> {
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
    fn parse_rule_nongen() -> Result<(), ParseRuleError> {
        let rule = LifeGenRule::parse_rule("B3/S23")?;

        let b: Vec<u8> = rule.iter_b().collect();
        let s: Vec<u8> = rule.iter_s().collect();
        let gen = rule.gen();

        assert_eq!(b, vec![3]);
        assert_eq!(s, vec![2, 3]);
        assert_eq!(gen, 2);

        assert_eq!(rule.to_string_bsg(), "B3/S23");
        assert_eq!(rule.to_string_sbg(), "23/3");
        assert_eq!(rule.to_string_catagolue(), "b3s23");

        let non_gen = LifeRule::parse_rule("B3/S23")?;
        assert_eq!(LifeRule::try_from(rule).unwrap(), non_gen);

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
