use crate::{error::ParseRuleError, parse_rules::Neighborhood};
use std::iter::Peekable;

#[derive(Clone, Copy, Debug)]
pub struct Lifelike;

impl Neighborhood for Lifelike {
    const SUFFIX: Option<char> = None;

    fn parse_bs<I>(chars: &mut Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
    where
        I: Iterator<Item = char>,
    {
        let mut bs = Vec::new();

        while let Some(&c) = chars.peek() {
            match c {
                c if c.is_digit(9) => {
                    chars.next();
                    bs.push(c.to_digit(9).unwrap() as u8);
                }
                '/' | 'S' | 's' => return Ok(bs),
                c => return Err(ParseRuleError::Unexpected(c)),
            }
        }
        Ok(bs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_rules::ParseBSRules;

    struct Rule {
        b: Vec<u8>,
        s: Vec<u8>,
    }

    impl ParseBSRules for Rule {
        type Neighborhood = Lifelike;

        fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
            Rule { b, s }
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
            Some(ParseRuleError::Unexpected('h'))
        );
        assert_eq!(
            Rule::parse_rule(&"B3/23").err(),
            Some(ParseRuleError::Missing('S'))
        );
        assert_eq!(
            Rule::parse_rule(&"B2e3-anq/S12-a3").err(),
            Some(ParseRuleError::Unexpected('e'))
        );
        assert_eq!(
            Rule::parse_rule(&"233").err(),
            Some(ParseRuleError::Missing('/'))
        );
        Ok(())
    }

    #[test]
    fn game_of_life() -> Result<(), ParseRuleError> {
        let life = Rule::parse_rule(&"B3/S23")?;
        for b in 0..=8_u8 {
            assert_eq!(life.b.contains(&b), [3].contains(&b));
        }
        for s in 0..=8_u8 {
            assert_eq!(life.s.contains(&s), [2, 3].contains(&s));
        }
        Ok(())
    }
}
