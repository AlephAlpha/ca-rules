//! Parsers for different types of rules.mod hex;

mod hex;
mod life;
mod neumann;
mod nthex;
mod ntlife;
mod ntneumann;

pub use hex::{ParseHex, ParseHexGen};
pub use life::{ParseLife, ParseLifeGen};
pub use neumann::{ParseNeumann, ParseNeumannGen};
pub use nthex::{ParseNtHex, ParseNtHexGen};
pub use ntlife::{ParseNtLife, ParseNtLifeGen};
pub use ntneumann::{ParseNtNeumann, ParseNtNeumannGen};

/// A helper struct to represent Generations rules.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Gen<T> {
    rule: T,
    gen: usize,
}
