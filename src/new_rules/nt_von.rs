//! Non-totalistic rules with von Neumann neighborhood.

use crate::traits::ParseMapRule;
use fixedbitset::FixedBitSet;

/// Non-totalistic rules with
/// [von Neumann neighborhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NtVonRule {
    pub(crate) data: FixedBitSet,
}

impl NtVonRule {
    /// The rule data, in a [`FixedBitSet`].
    pub fn data(&self) -> &FixedBitSet {
        &self.data
    }
}

impl ParseMapRule for NtVonRule {
    const DATA_SIZE: usize = 1 << 5;

    fn from_data(data: FixedBitSet) -> Self {
        Self { data }
    }
}

impl Default for NtVonRule {
    #[inline]
    fn default() -> Self {
        Self {
            data: FixedBitSet::with_capacity(1 << 5),
        }
    }
}
