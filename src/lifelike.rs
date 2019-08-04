use crate::{error::ParseRuleError, parse_rules::Neighborhood};
use std::{iter::Peekable, str::Chars};

pub struct Lifelike;

impl Neighborhood for Lifelike {
    fn parse_bs(chars: &mut Peekable<Chars>) -> Result<Vec<u8>, ParseRuleError> {
        let mut bs = Vec::new();

        while let Some(&c) = chars.peek() {
            match c {
                c if c.is_digit(9) => {
                    chars.next();
                    bs.push(c.to_string().parse::<u8>().unwrap());
                }
                '/' | 'S' | 's' => return Ok(bs),
                _ => return Err(ParseRuleError::MissingNumber),
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
        Ok(())
    }

    #[test]
    fn invalid_rules() -> Result<(), ParseRuleError> {
        assert!(Rule::parse_rule(&"B3/S23H").is_err());
        assert!(Rule::parse_rule(&"B2e3-anq/S12-a3").is_err());
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
