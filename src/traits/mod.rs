mod parse;
mod print;
pub mod total;

pub use parse::{ParseGenRule, ParseRule};
pub use print::{PrintGenRule, PrintRule};
pub use total::{Totalistic, TotalisticGen};
