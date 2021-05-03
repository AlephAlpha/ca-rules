use crate::error::ParseRuleError;
use std::iter::Peekable;

/// B or S.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bs {
    B,
    S,
}

/// A parser for numbers.
pub fn parse_num<I>(chars: &mut Peekable<I>) -> Result<u32, ParseRuleError>
where
    I: Iterator<Item = char>,
{
    let mut n = 0_u32;
    if chars.peek().is_none() || !chars.peek().unwrap().is_digit(10) {
        return Err(ParseRuleError::MissingNumber);
    }
    while let Some(&c) = chars.peek().filter(|c| c.is_digit(10)) {
        chars.next();
        n = n
            .checked_mul(10)
            .ok_or(ParseRuleError::GenOverflow)?
            .checked_add(c.to_digit(10).unwrap() as u32)
            .ok_or(ParseRuleError::GenOverflow)?;
    }
    Ok(n)
}
