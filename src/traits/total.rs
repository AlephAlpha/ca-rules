//! A trait for totalistic rules.

use super::{ParseGenRule, ParseRule, PrintGenRule, PrintRule};
use crate::util::Bs::{self, B, S};
use fixedbitset::{FixedBitSet, Ones};
use std::iter::Peekable;

/// An iterator over the `b` data of a totalistic rule.
pub struct IterB<'a> {
    iter: Ones<'a>,
    nbhd_size: usize,
}

impl<'a> IterB<'a> {
    #[inline]
    fn new(iter: Ones<'a>, nbhd_size: usize) -> Self {
        Self { iter, nbhd_size }
    }
}

impl<'a> Iterator for IterB<'a> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let nbhd_size = self.nbhd_size;
        self.iter
            .find_map(|bit| (bit < nbhd_size).then(|| bit as u8))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.nbhd_size))
    }
}

/// An iterator over the `b` data of a totalistic rule.
pub struct IterS<'a> {
    iter: Ones<'a>,
    nbhd_size: usize,
}

impl<'a> IterS<'a> {
    #[inline]
    fn new(iter: Ones<'a>, nbhd_size: usize) -> Self {
        Self { iter, nbhd_size }
    }
}

impl<'a> Iterator for IterS<'a> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let nbhd_size = self.nbhd_size;
        self.iter
            .find_map(|bit| (bit >= nbhd_size).then(|| (bit - nbhd_size) as u8))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.nbhd_size))
    }
}

/// A trait for totalistic rules.
///
/// The `b` / `s` data of these rules consists of numbers of live neighbors
/// that cause a cell to be born / survive.
pub trait Totalistic: Sized {
    /// Number of cells in the neighborhood, including the cell itself.
    const NBHD_SIZE: usize;

    /// The suffix of the rule string.
    const SUFFIX: Option<char>;

    /// Generate a new rule from the `b`/`s` data in a [`FixedBitSet`].
    fn from_data(data: FixedBitSet) -> Self;

    /// The `b`/`s` data, in a [`FixedBitSet`].
    fn data(&self) -> &FixedBitSet;

    /// A new rule with empty `b`/`s` data, i.e., `B/S`.
    #[inline]
    fn new() -> Self {
        let data = FixedBitSet::with_capacity(2 * Self::NBHD_SIZE);
        Self::from_data(data)
    }

    /// Whether the rule contains this `b` data.
    #[inline]
    fn contains_b(&self, b: u8) -> bool {
        self.data().contains(b as usize)
    }

    /// Whether the rule contains this `s` data.
    #[inline]
    fn contains_s(&self, s: u8) -> bool {
        self.data().contains(s as usize + Self::NBHD_SIZE)
    }

    /// An iterator over the `b` data of the rule.
    #[inline]
    fn iter_b(&self) -> IterB<'_> {
        IterB::new(self.data().ones(), Self::NBHD_SIZE)
    }

    /// An iterator over the `s` data of the rule.
    #[inline]
    fn iter_s(&self) -> IterS<'_> {
        IterS::new(self.data().ones(), Self::NBHD_SIZE)
    }
}

impl<R: Totalistic> ParseRule for R {
    const DATA_SIZE: usize = 2 * Self::NBHD_SIZE;
    const SUFFIX: Option<char> = <Self as Totalistic>::SUFFIX;

    #[inline]
    fn read_bs<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>, bs: Bs)
    where
        I: Iterator<Item = char>,
    {
        while let Some(d) = chars
            .peek()
            .and_then(|c| c.to_digit(Self::NBHD_SIZE as u32))
        {
            chars.next();
            match bs {
                B => data.insert(d as usize),
                S => data.insert((d as usize) + Self::NBHD_SIZE),
            }
        }
    }

    #[inline]
    fn from_data(data: FixedBitSet) -> Self {
        <Self as Totalistic>::from_data(data)
    }
}

impl<R: Totalistic> PrintRule for R {
    const SUFFIX: Option<char> = <Self as Totalistic>::SUFFIX;

    #[inline]
    fn write_bs(&self, string: &mut String, bs: Bs) {
        match bs {
            B => {
                for b in self.iter_b() {
                    string.push(char::from_digit(b as u32, Self::NBHD_SIZE as u32).unwrap());
                }
            }
            S => {
                for s in self.iter_s() {
                    string.push(char::from_digit(s as u32, Self::NBHD_SIZE as u32).unwrap());
                }
            }
        }
    }
}

/// A trait for totalistic Generations rules.
pub trait TotalisticGen: Sized {
    /// Number of cells in the neighborhood, including the cell itself.
    const NBHD_SIZE: usize;

    /// The suffix of the rule string.
    const SUFFIX: Option<char>;

    /// Generate a new rule from the `b`/`s` data in a [`FixedBitSet`],
    /// and the generation number.
    fn from_data(data: FixedBitSet, gen: u32) -> Self;

    /// The `b`/`s` data, in a [`FixedBitSet`].
    fn data(&self) -> &FixedBitSet;

    /// The generation number.
    fn gen(&self) -> u32;

    /// A new rule with empty `b`/`s` data, i.e., `B/S`.
    #[inline]
    fn new(gen: u32) -> Self {
        let data = FixedBitSet::with_capacity(2 * Self::NBHD_SIZE);
        Self::from_data(data, gen)
    }

    /// Whether the rule contains this `b` data.
    #[inline]
    fn contains_b(&self, b: u8) -> bool {
        (b as usize) < Self::NBHD_SIZE && self.data().contains(b as usize)
    }

    /// Whether the rule contains this `s` data.
    #[inline]
    fn contains_s(&self, s: u8) -> bool {
        (s as usize) < Self::NBHD_SIZE && self.data().contains(s as usize + 7)
    }

    /// An iterator over the `b` data of the rule.
    #[inline]
    fn iter_b(&self) -> IterB<'_> {
        IterB::new(self.data().ones(), Self::NBHD_SIZE)
    }

    /// An iterator over the `s` data of the rule.
    #[inline]
    fn iter_s(&self) -> IterS<'_> {
        IterS::new(self.data().ones(), Self::NBHD_SIZE)
    }
}

impl<R: TotalisticGen> ParseGenRule for R {
    const DATA_SIZE: usize = 2 * Self::NBHD_SIZE;
    const SUFFIX: Option<char> = <Self as TotalisticGen>::SUFFIX;

    #[inline]
    fn read_bs<I>(data: &mut FixedBitSet, chars: &mut Peekable<I>, bs: Bs)
    where
        I: Iterator<Item = char>,
    {
        while let Some(d) = chars
            .peek()
            .and_then(|c| c.to_digit(Self::NBHD_SIZE as u32))
        {
            chars.next();
            match bs {
                B => data.insert(d as usize),
                S => data.insert((d as usize) + Self::NBHD_SIZE),
            }
        }
    }

    #[inline]
    fn from_data(data: FixedBitSet, gen: u32) -> Self {
        <Self as TotalisticGen>::from_data(data, gen)
    }
}

impl<R: TotalisticGen> PrintGenRule for R {
    const SUFFIX: Option<char> = <Self as TotalisticGen>::SUFFIX;

    #[inline]
    fn get_gen(&self) -> u32 {
        <Self as TotalisticGen>::gen(&self)
    }

    #[inline]
    fn write_bs(&self, string: &mut String, bs: Bs) {
        match bs {
            B => {
                for b in self.iter_b() {
                    string.push(char::from_digit(b as u32, Self::NBHD_SIZE as u32).unwrap());
                }
            }
            S => {
                for s in self.iter_s() {
                    string.push(char::from_digit(s as u32, Self::NBHD_SIZE as u32).unwrap());
                }
            }
        }
    }
}
