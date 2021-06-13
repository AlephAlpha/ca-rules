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

/// [Totalistic hexagonal rules](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood).
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::HexRule;
/// use ca_rules::traits::*;
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

impl Totalistic for HexRule {
    const NBHD_SIZE: usize = 7;

    const SUFFIX: Option<char> = Some('H');

    #[inline]
    fn from_data(data: FixedBitSet) -> Self {
        Self { data }
    }

    #[inline]
    fn data(&self) -> &FixedBitSet {
        &self.data
    }
}

impl Default for HexRule {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for HexRule {
    type Err = ParseRuleError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl Display for HexRule {
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
        let rule = HexRule::parse_rule("B2/S34H")?;

        let b: Vec<u8> = rule.iter_b().collect();
        let s: Vec<u8> = rule.iter_s().collect();

        assert_eq!(b, vec![2]);
        assert_eq!(s, vec![3, 4]);

        for i in 0..HexRule::NBHD_SIZE as u8 {
            assert_eq!(b.contains(&i), rule.contains_b(i));
            assert_eq!(s.contains(&i), rule.contains_s(i));
        }

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
