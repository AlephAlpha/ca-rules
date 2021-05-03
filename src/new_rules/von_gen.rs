//! Totalistic Generations rules with von Neumann neighborhood.

use crate::{
    error::{ConverRuleError, ParseRuleError},
    new_rules::von::{self, VonRule},
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

/// Totalistic [Generations](http://www.conwaylife.com/wiki/Generations) rules
/// with [von Neumann neighborhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::VonGenRule;
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

impl VonGenRule {
    /// Whether the rule contains this `b` data.
    pub fn contains_b(&self, b: u8) -> bool {
        self.data.contains(b as usize)
    }

    /// Whether the rule contains this `s` data.
    pub fn contains_s(&self, s: u8) -> bool {
        self.data.contains(s as usize + 5)
    }

    /// An iterator over the `b` data of the rule.
    pub fn iter_b(&self) -> impl Iterator<Item = u8> + '_ {
        self.data
            .ones()
            .filter_map(|bit| (bit < 5).then(|| bit as u8))
    }

    /// An iterator over the `s` data of the rule.
    pub fn iter_s(&self) -> impl Iterator<Item = u8> + '_ {
        self.data
            .ones()
            .filter_map(|bit| (bit >= 5).then(|| (bit - 5) as u8))
    }

    /// The generation number.
    pub fn gen(&self) -> u32 {
        self.gen
    }
}

impl Default for VonGenRule {
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(10),
            gen: 2,
        }
    }
}

impl ParseGenRule for VonGenRule {
    const DATA_SIZE: usize = 10;
    const SUFFIX: Option<char> = Some('V');

    fn read_bs<I>(data: &mut FixedBitSet, chars: &mut std::iter::Peekable<I>, bs: Bs)
    where
        I: Iterator<Item = char>,
    {
        von::read_bs(data, chars, bs)
    }

    fn from_data(data: FixedBitSet, gen: u32) -> Self {
        Self { data, gen }
    }
}

impl FromStr for VonGenRule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl PrintGenRule for VonGenRule {
    const SUFFIX: Option<char> = Some('V');

    fn gen(&self) -> u32 {
        self.gen
    }

    fn write_bs(&self, string: &mut String, bs: Bs) {
        match bs {
            B => {
                for b in self.iter_b() {
                    string.push(char::from_digit(b as u32, 5).unwrap());
                }
            }
            S => {
                for s in self.iter_s() {
                    string.push(char::from_digit(s as u32, 5).unwrap());
                }
            }
        }
    }
}

impl Display for VonGenRule {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.to_string_sbg())
    }
}

impl From<VonRule> for VonGenRule {
    fn from(rule: VonRule) -> Self {
        Self {
            data: rule.data,
            gen: 2,
        }
    }
}

impl TryFrom<VonGenRule> for VonRule {
    type Error = ConverRuleError;

    fn try_from(rule: VonGenRule) -> Result<Self, Self::Error> {
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
}
