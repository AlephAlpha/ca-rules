use crate::{
    error::ParseRuleError,
    util::{
        self,
        Bs::{self, B, S},
    },
};
use fixedbitset::FixedBitSet;
use std::iter::Peekable;

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

/// A trait for printing non-Generations rules.
pub trait PrintRule {
    /// The suffix of the rule string.
    const SUFFIX: Option<char>;

    /// Writing `b`/`s` data.
    fn write_bs(&self, string: &mut String, bs: Bs);

    /// Print the rule in B/S notation, e.g. `B3/S23`.
    fn to_string_bs(&self) -> String {
        let mut string = String::new();

        string.push('B');
        self.write_bs(&mut string, B);
        string.push('/');
        string.push('S');
        self.write_bs(&mut string, S);
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_uppercase());
        }
        string
    }

    /// Print the rule in S/B notation, e.g. `23/3`.
    fn to_string_sb(&self) -> String {
        let mut string = String::new();

        self.write_bs(&mut string, S);
        string.push('/');
        self.write_bs(&mut string, B);
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_uppercase());
        }
        string
    }

    /// Print the rule in S/B notation, e.g. `b3s23`.
    fn to_string_catagolue(&self) -> String {
        let mut string = String::new();

        string.push('b');
        self.write_bs(&mut string, B);
        string.push('s');
        self.write_bs(&mut string, S);
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_lowercase());
        }
        string
    }
}

/// A trait for printing Generations rules.
pub trait PrintGenRule {
    /// The suffix of the rule string.
    const SUFFIX: Option<char>;

    /// Writing `b`/`s` data.
    fn write_bs(&self, string: &mut String, bs: Bs);

    // The generation number.
    fn gen(&self) -> u32;

    /// Print the rule in B/S/G notation, e.g. `B3/S23/G3`.
    fn to_string_bsg(&self) -> String {
        let mut string = String::new();

        string.push('B');
        self.write_bs(&mut string, B);
        string.push('/');
        string.push('S');
        self.write_bs(&mut string, S);
        string.push('/');
        string.push('G');
        string.push_str(&self.gen().to_string());
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_uppercase());
        }
        string
    }

    /// Print the rule in S/B/G notation, e.g. `23/3/3`.
    fn to_string_sbg(&self) -> String {
        let mut string = String::new();

        self.write_bs(&mut string, S);
        string.push('/');
        self.write_bs(&mut string, B);
        string.push('/');
        string.push_str(&self.gen().to_string());
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_uppercase());
        }
        string
    }

    /// Print the rule in Catagolue's notation, e.g. `g3b3s23`.
    fn to_string_catagolue(&self) -> String {
        let mut string = String::new();

        string.push('g');
        string.push_str(&self.gen().to_string());
        string.push('b');
        self.write_bs(&mut string, B);
        string.push('s');
        self.write_bs(&mut string, S);
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_lowercase());
        }
        string
    }
}
