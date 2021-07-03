use crate::error::ParseRuleError;
use fixedbitset::FixedBitSet;
use std::{convert::TryInto, iter::Peekable};

/// A parser for numbers.
fn parse_num<I>(chars: &mut Peekable<I>) -> Result<u32, ParseRuleError>
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

/// A trait for parsing non-Generations rules.
pub trait ParseRule: Sized {
    /// Size of the data in a [`FixedBitSet`].
    const DATA_SIZE: usize;

    /// The suffix of the rule string.
    const SUFFIX: Option<char>;

    /// Reading `b` data.
    fn read_b<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>)
    where
        I: Iterator<Item = char>;

    /// Reading `s` data.
    fn read_s<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>)
    where
        I: Iterator<Item = char>;

    /// Generate a new rule from the `b`/`s` data in a [`FixedBitSet`].
    fn from_data(data: FixedBitSet) -> Self;

    /// A parser for the struct.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError> {
        let mut chars = input.chars().peekable();
        let mut data = FixedBitSet::with_capacity(Self::DATA_SIZE);

        if matches!(chars.peek(), Some('B') | Some('b')) {
            // Rule strings using B/S notation
            chars.next();
            Self::read_b(&mut data, &mut chars);
            if let Some('/') = chars.peek() {
                chars.next();
            }
            if !matches!(chars.next(), Some('S') | Some('s')) {
                return Err(ParseRuleError::Missing('S'));
            }
            Self::read_s(&mut data, &mut chars);
        } else {
            // Rule strings using S/B notation
            Self::read_s(&mut data, &mut chars);
            if chars.next() != Some('/') {
                return Err(ParseRuleError::Missing('/'));
            }
            Self::read_b(&mut data, &mut chars);
        }

        if let Some(suffix) = Self::SUFFIX {
            if chars.next().map(|c| c.to_ascii_uppercase()) != Some(suffix.to_ascii_uppercase()) {
                return Err(ParseRuleError::Missing(suffix));
            }
        }

        if chars.next().is_some() {
            Err(ParseRuleError::ExtraJunk)
        } else {
            Ok(Self::from_data(data))
        }
    }
}

/// A trait for parsing Generations rules.
pub trait ParseGenRule: Sized {
    /// Size of the data in a [`FixedBitSet`].
    const DATA_SIZE: usize;

    /// The suffix of the rule string.
    const SUFFIX: Option<char>;

    /// Reading `b` data.
    fn read_b<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>)
    where
        I: Iterator<Item = char>;

    /// Reading `s` data.
    fn read_s<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>)
    where
        I: Iterator<Item = char>;

    /// Generate a new rule from the `b`/`s` data in a [`FixedBitSet`] and the generation.
    fn from_data(data: FixedBitSet, gen: u32) -> Self;

    /// A parser for the rule.
    fn parse_rule(input: &str) -> Result<Self, ParseRuleError> {
        let mut chars = input.chars().peekable();
        let mut data = FixedBitSet::with_capacity(Self::DATA_SIZE);
        let mut gen = 2;

        match chars.peek() {
            // Rule strings using B/S/G notation
            Some('B') | Some('b') => {
                chars.next();
                Self::read_b(&mut data, &mut chars);
                if let Some('/') = chars.peek() {
                    chars.next();
                }
                if !matches!(chars.next(), Some('S') | Some('s')) {
                    return Err(ParseRuleError::Missing('S'));
                }
                Self::read_s(&mut data, &mut chars);
                match chars.peek() {
                    Some('/') => {
                        chars.next();
                        if matches!(chars.peek(), Some('C') | Some('c') | Some('G') | Some('g')) {
                            chars.next();
                        }
                        gen = parse_num(&mut chars)?;
                    }
                    Some('C') | Some('c') | Some('G') | Some('g') => {
                        chars.next();
                        gen = parse_num(&mut chars)?;
                    }
                    _ => (),
                }
            }

            // Rule strings using G/B/S notation
            Some('C') | Some('c') | Some('G') | Some('g') => {
                chars.next();
                gen = parse_num(&mut chars)?;
                if let Some('/') = chars.peek() {
                    chars.next();
                }
                if !matches!(chars.next(), Some('B') | Some('b')) {
                    return Err(ParseRuleError::Missing('B'));
                }
                Self::read_b(&mut data, &mut chars);
                if let Some('/') = chars.peek() {
                    chars.next();
                }
                if !matches!(chars.next(), Some('S') | Some('s')) {
                    return Err(ParseRuleError::Missing('S'));
                }
                Self::read_s(&mut data, &mut chars);
            }

            // Rule strings using S/B/G notation
            _ => {
                Self::read_s(&mut data, &mut chars);
                if chars.next() != Some('/') {
                    return Err(ParseRuleError::Missing('/'));
                }
                Self::read_b(&mut data, &mut chars);
                if let Some('/') = chars.peek() {
                    chars.next();
                    gen = parse_num(&mut chars)?;
                }
            }
        }

        if let Some(suffix) = Self::SUFFIX {
            if chars
                .next()
                .map_or(true, |c| !c.eq_ignore_ascii_case(&suffix))
            {
                return Err(ParseRuleError::Missing(suffix));
            }
        }

        if gen < 2 {
            Err(ParseRuleError::GenLessThan2)
        } else if chars.next().is_some() {
            Err(ParseRuleError::ExtraJunk)
        } else {
            Ok(Self::from_data(data, gen))
        }
    }
}

/// A trait for parsing non-Generations rules with
/// [MAP string](http://golly.sourceforge.net/Help/Algorithms/QuickLife.html#map).
pub trait ParseMapRule: Sized {
    /// Size of the data in a [`FixedBitSet`]. Should be a multiple of 4.
    const DATA_SIZE: usize;

    /// Generate a new rule from the data in a [`FixedBitSet`].
    fn from_data(data: FixedBitSet) -> Self;

    /// A parser for MAP strings.
    fn parse_rule_map(input: &str) -> Result<Self, ParseRuleError> {
        assert_eq!(
            Self::DATA_SIZE % 4,
            0,
            "`DATA_SIZE` should be a multiple of 4."
        );

        if !input.starts_with("MAP") {
            return Err(ParseRuleError::NotMapRule);
        }
        let bytes = base64::decode(&input[3..]).map_err(|_| ParseRuleError::Base64Error)?;
        if bytes.len() * 8 != Self::DATA_SIZE {
            return Err(ParseRuleError::InvalidLength);
        }

        let blocks = bytes
            .chunks_exact(4)
            .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()).reverse_bits());

        let data = FixedBitSet::with_capacity_and_blocks(Self::DATA_SIZE, blocks);

        Ok(Self::from_data(data))
    }
}
