use super::Gen;
use crate::ParseRuleError;

rule_struct!(Neumann);

impl Neumann {
    parse_bs!(4);
    parse_rule!('V');
}

/// A trait for parsing rules with
/// [von Neumann neighbourhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// # Examples
///
/// ```
/// use ca_rules::rules::ParseNeumann;
///
/// struct Rule {
///     b: Vec<u8>,
///     s: Vec<u8>,
/// }
///
/// impl ParseNeumann for Rule {
///     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
///         Rule { b, s }
///     }
/// }
///
/// let life = Rule::parse_rule(&"B2/S013V").unwrap();
///
/// for b in 0..=4 {
///     assert_eq!(life.b.contains(&b), [2].contains(&b));
/// }
///
/// for s in 0..=4 {
///     assert_eq!(life.s.contains(&s), [0, 1, 3].contains(&s));
/// }
/// ```
pub trait ParseNeumann {
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Neumann { b, s } = Neumann::parse_rule(input)?;
        Ok(Self::from_bs(b, s))
    }
}

/// A trait for parsing [Generations](http://www.conwaylife.com/wiki/Generations) rules
/// with [von Neumann neighbourhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
///
/// The `b` / `s` data of this type of rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
///
/// Examples will be added later.
pub trait ParseNeumannGen {
    fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self;

    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Gen {
            rule: Neumann { b, s },
            gen,
        } = Neumann::parse_rule_gen(input)?;
        Ok(Self::from_bsg(b, s, gen))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rule;

    impl ParseNeumann for Rule {
        fn from_bs(_b: Vec<u8>, _s: Vec<u8>) -> Self {
            Rule
        }
    }

    #[test]
    fn valid_rules() -> Result<(), ParseRuleError> {
        Rule::parse_rule(&"B3/S23V")?;
        Rule::parse_rule(&"B3S23V")?;
        Rule::parse_rule(&"b3s23v")?;
        Rule::parse_rule(&"23/3V")?;
        Rule::parse_rule(&"23/v")?;
        Ok(())
    }

    #[test]
    fn invalid_rules() -> Result<(), ParseRuleError> {
        assert_eq!(
            Rule::parse_rule(&"B3/S23va").err(),
            Some(ParseRuleError::ExtraJunk)
        );
        assert_eq!(
            Rule::parse_rule(&"B3V/S23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule(&"B3/S23").err(),
            Some(ParseRuleError::Missing('V'))
        );
        assert_eq!(
            Rule::parse_rule(&"B3/S25V").err(),
            Some(ParseRuleError::Missing('V'))
        );
        assert_eq!(
            Rule::parse_rule(&"233v").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }
}
