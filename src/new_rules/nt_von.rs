//! Non-totalistic rules with von Neumann neighborhood.

use crate::{
    error::ParseRuleError,
    new_rules::VonRule,
    traits::{ParseMapRule, Totalistic},
    ParseRule,
};
use fixedbitset::FixedBitSet;
use std::str::FromStr;

/// Non-totalistic rules with
/// [von Neumann neighborhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::NtVonRule;
///
/// let rule: NtVonRule = "MAPHmlphg".parse().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NtVonRule {
    pub(crate) data: FixedBitSet,
}

impl NtVonRule {
    /// The rule data, in a [`FixedBitSet`].
    pub fn data(&self) -> &FixedBitSet {
        &self.data
    }
}

impl ParseMapRule for NtVonRule {
    const DATA_SIZE: usize = 1 << 5;

    fn from_data(data: FixedBitSet) -> Self {
        Self { data }
    }
}

impl Default for NtVonRule {
    #[inline]
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(1 << 5),
        }
    }
}

impl FromStr for NtVonRule {
    type Err = ParseRuleError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        VonRule::parse_rule(s).map(Self::from).or_else(|err| {
            Self::parse_rule_map(s).map_err(|err_map| {
                if let ParseRuleError::NotMapRule = err_map {
                    err
                } else {
                    err_map
                }
            })
        })
    }
}

impl From<VonRule> for NtVonRule {
    fn from(rule: VonRule) -> Self {
        let mut data = FixedBitSet::with_capacity(1 << 5);
        for i in 0_usize..1 << 5 {
            let condition = if i & (1 << 2) == 0 {
                rule.contains_b(i.count_ones() as u8)
            } else {
                rule.contains_s(i.count_ones() as u8 - 1)
            };
            if condition {
                data.insert(i);
            }
        }
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::ParseRule;

    #[test]
    fn parse_rule_totalistic() -> Result<(), ParseRuleError> {
        let rule = NtVonRule::parse_rule_map("MAPHmlphg")?;

        let totalistic = VonRule::parse_rule("B2/S013V")?;

        assert_eq!(NtVonRule::from(totalistic), rule);

        Ok(())
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        NtVonRule::from_str("B3/S23V")?;
        NtVonRule::from_str("B3S23V")?;
        NtVonRule::from_str("b3s23v")?;
        NtVonRule::from_str("23/3V")?;
        NtVonRule::from_str("23/v")?;
        NtVonRule::from_str("MAPHmlphg")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() {
        assert_eq!(
            NtVonRule::from_str("B3/S23va").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            NtVonRule::from_str("B3V/S23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            NtVonRule::from_str("B3/S23").err(),
            Some(ParseRuleError::Missing('V'))
        );
        assert_eq!(
            NtVonRule::from_str("B3/S25V").err(),
            Some(ParseRuleError::Missing('V'))
        );
        assert_eq!(
            NtVonRule::from_str("233v").err(),
            Some(ParseRuleError::Missing('/'))
        );
        assert_eq!(
            NtVonRule::from_str("MAPFgFoF2gXgH5oF4B+gH4A6A").err(),
            Some(ParseRuleError::InvalidLength)
        );
    }
}
