use super::Neighborhood;
use crate::error::ParseRuleError;
use std::iter::Peekable;

/// Neighborhood for [totalistic life-like rules](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton).
///
/// The `b` / `s` data of this neighborhood type consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::{neighborhood, ParseBSRules};
///
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseBSRules for Rule {
///     type Neighborhood = neighborhood::Lifelike;
///
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule(&"B3/S23").unwrap();
///
/// for b in 0..=8 {
///     assert_eq!(life.b.contains(&b), [3].contains(&b));
/// }
///
/// for s in 0..=8 {
///     assert_eq!(life.s.contains(&s), [2, 3].contains(&s));
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Lifelike;

impl Neighborhood for Lifelike {
    const SUFFIX: Option<char> = None;

    parse_bs_totalistic!(8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParseBSRules;

    struct Rule;

    impl ParseBSRules for Rule {
        type Neighborhood = Lifelike;

        fn from_bs(_b: Vec<u8>, _s: Vec<u8>) -> Self {
            Rule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule(&"B3/S23")?;
        Rule::parse_rule(&"B3S23")?;
        Rule::parse_rule(&"b3s23")?;
        Rule::parse_rule(&"23/3")?;
        Rule::parse_rule(&"23/")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() -> Result<(), ParseRuleError> {
        assert_eq!(
            Rule::parse_rule(&"B3/S23h").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            Rule::parse_rule(&"B3/23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule(&"B2e3-anq/S12-a3").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule(&"233").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }
}
