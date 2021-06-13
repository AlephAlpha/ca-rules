//! Non-totalistic rules with von Neumann neighborhood.

use crate::{
    new_rules::VonRule,
    traits::{ParseMapRule, Totalistic},
};
use fixedbitset::FixedBitSet;

/// Non-totalistic rules with
/// [von Neumann neighborhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
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
    use crate::{error::ParseRuleError, traits::ParseRule};

    #[test]
    fn parse_rule_totalistic() -> Result<(), ParseRuleError> {
        let rule = NtVonRule::parse_rule_map("MAPHmlphg")?;

        let totalistic = VonRule::parse_rule("B2/S013V")?;

        assert_eq!(NtVonRule::from(totalistic), rule);

        Ok(())
    }
}
