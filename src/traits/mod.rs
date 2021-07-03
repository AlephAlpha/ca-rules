mod parse;
mod print;
pub mod total;

pub use parse::{ParseGenRule, ParseMapRule, ParseRule};
pub use print::{PrintGenRule, PrintMapRule, PrintRule};
pub use total::{Totalistic, TotalisticGen};
