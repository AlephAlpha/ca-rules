use crate::{
    error::ParseRuleError,
    util::{
        self,
        Bs::{self, B, S},
    },
};
use fixedbitset::FixedBitSet;
use std::{convert::TryInto, iter::Peekable};

/// A trait for parsing non-Generations rules.
pub trait ParseRule: Sized {
    /// Size of the data in a [`FixedBitSet`].
    const DATA_SIZE: usize;

    /// The suffix of the rule string.
    const SUFFIX: Option<char>;

    /// Reading `b`/`s` data.
    fn read_bs<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>, bs: Bs)
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
            Self::read_bs(&mut data, &mut chars, B);
            if let Some('/') = chars.peek() {
                chars.next();
            }
            if !matches!(chars.next(), Some('S') | Some('s')) {
                return Err(ParseRuleError::Missing('S'));
            }
            Self::read_bs(&mut data, &mut chars, S);
        } else {
            // Rule strings using S/B notation
            Self::read_bs(&mut data, &mut chars, S);
            if chars.next() != Some('/') {
                return Err(ParseRuleError::Missing('/'));
            }
            Self::read_bs(&mut data, &mut chars, B);
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

    /// Reading `b`/`s` data.
    fn read_bs<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>, bs: Bs)
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
                Self::read_bs(&mut data, &mut chars, B);
                if let Some('/') = chars.peek() {
                    chars.next();
                }
                if !matches!(chars.next(), Some('S') | Some('s')) {
                    return Err(ParseRuleError::Missing('S'));
                }
                Self::read_bs(&mut data, &mut chars, S);
                match chars.peek() {
                    Some('/') => {
                        chars.next();
                        if matches!(chars.peek(), Some('C') | Some('c') | Some('G') | Some('g')) {
                            chars.next();
                        }
                        gen = util::parse_num(&mut chars)?;
                    }
                    Some('C') | Some('c') | Some('G') | Some('g') => {
                        chars.next();
                        gen = util::parse_num(&mut chars)?;
                    }
                    _ => (),
                }
            }

            // Rule strings using G/B/S notation
            Some('C') | Some('c') | Some('G') | Some('g') => {
                chars.next();
                gen = util::parse_num(&mut chars)?;
                if let Some('/') = chars.peek() {
                    chars.next();
                }
                if !matches!(chars.next(), Some('B') | Some('b')) {
                    return Err(ParseRuleError::Missing('B'));
                }
                Self::read_bs(&mut data, &mut chars, B);
                if let Some('/') = chars.peek() {
                    chars.next();
                }
                if !matches!(chars.next(), Some('S') | Some('s')) {
                    return Err(ParseRuleError::Missing('S'));
                }
                Self::read_bs(&mut data, &mut chars, S);
            }

            // Rule strings using S/B/G notation
            _ => {
                Self::read_bs(&mut data, &mut chars, S);
                if chars.next() != Some('/') {
                    return Err(ParseRuleError::Missing('/'));
                }
                Self::read_bs(&mut data, &mut chars, B);
                if let Some('/') = chars.peek() {
                    chars.next();
                    gen = util::parse_num(&mut chars)?;
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
/// [MAP string](https://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton).
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
