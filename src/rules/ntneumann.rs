use super::{
    neumann::{ParseNeumann, ParseNeumannGen},
    Gen,
};
use crate::ParseRuleError;

rule_struct!(NtNeumann);

impl ParseNeumann for NtNeumann {
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
        let mut new_b = Vec::new();
        let mut new_s = Vec::new();
        for i in 0_u8..=0x0f {
            let j = i.count_ones() as u8;
            if b.contains(&j) {
                new_b.push(i);
            }
            if s.contains(&j) {
                new_s.push(i);
            }
        }
        NtNeumann::from_bs(new_b, new_s)
    }
}

impl ParseNeumannGen for Gen<NtNeumann> {
    fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self {
        let mut new_b = Vec::new();
        let mut new_s = Vec::new();
        for i in 0_u8..=0x0f {
            let j = i.count_ones() as u8;
            if b.contains(&j) {
                new_b.push(i);
            }
            if s.contains(&j) {
                new_s.push(i);
            }
        }
        NtNeumann::from_bsg(new_b, new_s, gen)
    }
}

/// A trait for parsing non-totalistic rules
/// [von Neumann neighbourhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
///
/// The `b` / `s` data of this type of rules consists of possible combinations of
/// states of the 6 neighbors, represented by an 8-bit binary number,
/// that cause a cell to be born / survive.
///
/// For example, the following neighborhood is represented by the number `10 = 0b1010`:
/// ```plaintext
///   1
/// 0 _ 1
///   0
/// ```
///
/// For now, this parser only supports rule strings for totalistic rules, since There is not yet
/// a generally recognized notation for isotropic non-totalistic von Neumann neighbourhood.
/// [MAP notation](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
/// will be supported in the future.
pub trait ParseNtNeumann {
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self;

    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let NtNeumann { b, s } = ParseNeumann::parse_rule(input)?;
        Ok(Self::from_bs(b, s))
    }
}

/// A trait for parsing [non-totalistic hexagonal](http://www.conwaylife.com/wiki/Neumannagonal_neighbourhood)
/// [Generations](http://www.conwaylife.com/wiki/Generations) rules.
///
/// The `b` / `s` data of this type of rules consists of possible combinations of
/// states of the 6 neighbors, represented by an 8-bit binary number,
/// that cause a cell to be born / survive.
///
/// For example, the following neighborhood is represented by the number `42 = 0b101010`:
/// ```plaintext
///  1 0
/// 1 _ 0
///  1 0
/// ```
///
/// Examples will be added later.
pub trait ParseNtNeumannGen {
    fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self;

    fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
    where
        Self: Sized,
    {
        let Gen {
            rule: NtNeumann { b, s },
            gen,
        } = ParseNeumannGen::parse_rule(input)?;
        Ok(Self::from_bsg(b, s, gen))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rule;

    impl ParseNtNeumann for Rule {
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
