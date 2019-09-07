use crate::ParseRuleError;

rule_struct!(Life);

impl Life {
    parse_bs!(8);
    parse_rule!();
}

/// A trait for parsing [totalistic life-like rules](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton).
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::rules::ParseLife;
///
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseLife for Rule {
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
pub trait ParseLife {
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Life { b, s } = Life::parse_rule(input)?;
        Ok(Self::from_bs(b, s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rule;

    impl ParseLife for Rule {
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
