//! Totalistic rules with von Neumann neighborhood.

use crate::{
    error::ParseRuleError,
    traits::{ParseRule, PrintRule},
    util::Bs::{self, B, S},
};
use fixedbitset::FixedBitSet;
use std::{
    char,
    fmt::{self, Display, Formatter},
    iter::Peekable,
    str::FromStr,
};

/// Reading `b`/`s` data.
pub(crate) fn read_bs<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>, bs: Bs)
where
    I: Iterator<Item = char>,
{
    while let Some(d) = chars.peek().and_then(|c| c.to_digit(5)) {
        chars.next();
        match bs {
            B => data.insert(d as usize),
            S => data.insert((d as usize) + 5),
        }
    }
}

/// Totalistic rules with
/// [von Neumann neighborhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::new_rules::VonRule;
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

impl VonRule {
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
}

impl Default for VonRule {
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(10),
        }
    }
}

impl ParseRule for VonRule {
    const DATA_SIZE: usize = 10;
    const SUFFIX: Option<char> = Some('V');

    fn read_bs<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>, bs: Bs)
    where
        I: Iterator<Item = char>,
    {
        read_bs(data, chars, bs)
    }

    fn from_data(data: FixedBitSet) -> Self {
        Self { data }
    }
}

impl FromStr for VonRule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_rule(s)
    }
}

impl PrintRule for VonRule {
    const SUFFIX: Option<char> = Some('V');

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

impl Display for VonRule {
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
