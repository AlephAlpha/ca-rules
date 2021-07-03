//! Totalistic Generations rules with von Neumann neighborhood.

use crate::{
    error::{ConvertRuleError, ParseRuleError},
    new_rules::von::VonRule,
    traits::{ParseGenRule, PrintGenRule, TotalisticGen},
};
use fixedbitset::FixedBitSet;
use std::{
    char,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Totalistic [Generations](http://www.conwaylife.com/wiki/Generations) rules
/// with [von Neumann neighborhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::VonGenRule;
/// use ca_rules::traits::*;
///
/// let rule: VonGenRule = "g3b2s013V".parse().unwrap();
///
/// let b: Vec<u8> = rule.iter_b().collect();
/// let s: Vec<u8> = rule.iter_s().collect();
/// let gen = rule.gen();
///
/// assert_eq!(b, vec![2]);
/// assert_eq!(s, vec![0,1, 3]);
/// assert_eq!(gen, 3);
///
/// assert_eq!(rule.to_string(), "013/2/3V");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VonGenRule {
    data: FixedBitSet,
    gen: u32,
}

impl TotalisticGen for VonGenRule {
    const NBHD_SIZE: usize = 5;

    const SUFFIX: Option<char> = Some('V');

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

impl Default for VonGenRule {
    #[inline]
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(10),
            gen: 2,
        }
    }
}

impl FromStr for VonGenRule {
    type Err = ParseRuleError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}
impl Display for VonGenRule {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.gen() != 2 {
            f.write_str(&self.to_string_sbg())
        } else {
            f.write_str(&self.to_string_bsg())
        }
    }
}

impl From<VonRule> for VonGenRule {
    #[inline]
    fn from(rule: VonRule) -> Self {
        Self {
            data: rule.data,
            gen: 2,
        }
    }
}

impl TryFrom<VonGenRule> for VonRule {
    type Error = ConvertRuleError;

    #[inline]
    fn try_from(rule: VonGenRule) -> Result<Self, Self::Error> {
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
        let rule = VonGenRule::parse_rule("013/2/3V")?;

        let b: Vec<u8> = rule.iter_b().collect();
        let s: Vec<u8> = rule.iter_s().collect();
        let gen = rule.gen();

        assert_eq!(b, vec![2]);
        assert_eq!(s, vec![0, 1, 3]);
        assert_eq!(gen, 3);

        assert_eq!(rule.to_string_bsg(), "B2/S013/G3V");
        assert_eq!(rule.to_string_sbg(), "013/2/3V");
        assert_eq!(rule.to_string_catagolue(), "g3b2s013v");
        Ok(())
    }

    #[test]
    fn parse_rule_nongen() -> Result<(), ParseRuleError> {
        let rule = VonGenRule::parse_rule("B2/S013V")?;

        let b: Vec<u8> = rule.iter_b().collect();
        let s: Vec<u8> = rule.iter_s().collect();
        let gen = rule.gen();

        assert_eq!(b, vec![2]);
        assert_eq!(s, vec![0, 1, 3]);
        assert_eq!(gen, 2);

        assert_eq!(rule.to_string_bsg(), "B2/S013V");
        assert_eq!(rule.to_string_sbg(), "013/2V");
        assert_eq!(rule.to_string_catagolue(), "b2s013v");

        let non_gen = VonRule::parse_rule("B2/S013V")?;
        assert_eq!(VonRule::try_from(rule).unwrap(), non_gen);

        Ok(())
    }
}
