//! Totalistic rules with von Neumann neighborhood.

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

/// Totalistic rules with
/// [von Neumann neighborhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::VonRule;
/// use ca_rules::traits::*;
///
/// let rule: VonRule = "B2/S013V".parse().unwrap();
///
/// let b: Vec<u8> = rule.iter_b().collect();
/// let s: Vec<u8> = rule.iter_s().collect();
///
/// assert_eq!(b, vec![2]);
/// assert_eq!(s, vec![0, 1, 3]);
///
/// assert_eq!(rule.to_string(), "B2/S013V");
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VonRule {
    pub(crate) data: FixedBitSet,
}

impl Totalistic for VonRule {
    const NBHD_SIZE: usize = 5;

    const SUFFIX: Option<char> = Some('V');

    #[inline]
    fn from_data(data: FixedBitSet) -> Self {
        Self { data }
    }

    #[inline]
    fn data(&self) -> &FixedBitSet {
        &self.data
    }
}

impl Default for VonRule {
    #[inline]
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(10),
        }
    }
}

impl FromStr for VonRule {
    type Err = ParseRuleError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl Display for VonRule {
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
        let rule = VonRule::parse_rule("B2/S013V")?;

        let b: Vec<u8> = rule.iter_b().collect();
        let s: Vec<u8> = rule.iter_s().collect();

        assert_eq!(b, vec![2]);
        assert_eq!(s, vec![0, 1, 3]);

        assert_eq!(rule.to_string_bs(), "B2/S013V");
        assert_eq!(rule.to_string_sb(), "013/2V");
        assert_eq!(rule.to_string_catagolue(), "b2s013v");
        Ok(())
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        VonRule::parse_rule("B3/S23V")?;
        VonRule::parse_rule("B3S23V")?;
        VonRule::parse_rule("b3s23v")?;
        VonRule::parse_rule("23/3V")?;
        VonRule::parse_rule("23/v")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() {
        assert_eq!(
            VonRule::parse_rule("B3/S23va").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            VonRule::parse_rule("B3V/S23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            VonRule::parse_rule("B3/S23").err(),
            Some(ParseRuleError::Missing('V'))
        );
        assert_eq!(
            VonRule::parse_rule("B3/S25V").err(),
            Some(ParseRuleError::Missing('V'))
        );
        assert_eq!(
            VonRule::parse_rule("233v").err(),
            Some(ParseRuleError::Missing('/'))
        );
    }
}
