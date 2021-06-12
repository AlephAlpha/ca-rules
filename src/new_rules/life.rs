//! Totalistic hexagonal rules.

use crate::{
    error::ParseRuleError,
    traits::{ParseRule, PrintRule, Totalistic},
};
use fixedbitset::FixedBitSet;
use std::{
    char,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// [Totalistic life-like rules](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton).
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::LifeRule;
/// use ca_rules::traits::*;
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

impl Totalistic for LifeRule {
    const NBHD_SIZE: usize = 9;

    const SUFFIX: Option<char> = None;

    #[inline]
    fn from_data(data: FixedBitSet) -> Self {
        Self { data }
    }

    #[inline]
    fn data(&self) -> &FixedBitSet {
        &self.data
    }
}

impl Default for LifeRule {
    #[inline]
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(18),
        }
    }
}

impl FromStr for LifeRule {
    type Err = ParseRuleError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl Display for LifeRule {
    #[inline]
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
