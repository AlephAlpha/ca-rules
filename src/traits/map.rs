//! A trait for map rules.

use super::{ParseMapRule, PrintMapRule};
use fixedbitset::{FixedBitSet, Ones};

/// A trait for map rules.
///
/// The data of these rules consists of the neighboring cell states that
/// causes a cell to be alive in the next generation.
///
/// Here "neighboring cell states" means states of the cells in the
/// neighborhood, including the cell itself, represented by an
/// integer. For example, for life-like rules, each cell has a
/// neighborhood of size `9`, so there are `2 ^ 9 = 512` possible
/// combinations of states of the cells in the neighborhood.
/// Therefore, the neighboring cell states of such rules are
/// represented by a number from `0` to `511`.
pub trait MapRule: Sized {
    /// Number of cells in the neighborhood, including the cell itself.
    const NBHD_SIZE: usize;

    /// Generate a new rule from the data in a [`FixedBitSet`].
    fn from_data(data: FixedBitSet) -> Self;

    /// The data, in a [`FixedBitSet`].
    fn data(&self) -> &FixedBitSet;

    /// A new rule with empty data.
    #[inline]
    fn new() -> Self {
        let data = FixedBitSet::with_capacity(1 << Self::NBHD_SIZE);
        Self::from_data(data)
    }

    /// Whether a cell with this neighboring cell states would be alive
    /// in the next generation.
    #[inline]
    fn contains(&self, d: u8) -> bool {
        self.data().contains(d as usize)
    }

    /// An iterator over the neighboring cell states that causes a cell
    /// to be alive in the next generation.
    #[inline]
    fn iter(&self) -> Ones<'_> {
        self.data().ones()
    }
}

impl<R: MapRule> ParseMapRule for R {
    const DATA_SIZE: usize = 1 << Self::NBHD_SIZE;

    #[inline]
    fn from_data(data: FixedBitSet) -> Self {
        <Self as MapRule>::from_data(data)
    }
}

impl<R: MapRule> PrintMapRule for R {
    #[inline]
    fn data(&self) -> &FixedBitSet {
        <Self as MapRule>::data(self)
    }
}
