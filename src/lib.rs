//! Parsing rule strings of life-like and other cellular automata.
//!
//! Currently the following rules are supported:
//!
//! * [Totalistic Life-like](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton),
//!   e.g., `B3/S23`.
//! * [Isotropic non-totalistic Life-like](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton),
//!   e.g., `B2ci3ai4c8/S02ae3eijkq4iz5ar6i7e`.
//! * [Totalistic Hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood),
//!   e.g., `B2/S34H`.
//!
//! Please see [Life Wiki](www.conwaylife.com/wiki/Rulestring) for detailed definitions of
//! these rule strings.
//!
//! For the above rules, both [B/S notation](http://www.conwaylife.com/wiki/Rulestring#B.2FS_notation)
//! (`B3/S23`) and [S/B notaion](http://www.conwaylife.com/wiki/Rulestring#S.2FB_notation)
//! (`23/3`) are supported.
//!
//! # Example:
//! ```
//! use ca_rules::{Lifelike, ParseBSRules};
//!
//! // First you need to defind a struct for your rule:
//! #[derive(Debug, Eq, PartialEq)]
//! struct Rule {
//!     b: Vec<u8>,
//!     s: Vec<u8>,
//! }
//!
//! // Implement the ParseBSRules trait for your rule:
//! impl ParseBSRules for Rule {
//!     // Specify the neighborhood type:
//!     type Neighborhood = Lifelike;
//!
//!     // Implement a function to construct the rule from b and s data:
//!     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
//!         Rule { b, s }
//!     }
//! }
//!
//! // Then you can parse a rule string:
//! let life = Rule::parse_rule(&"B3/S23").unwrap();
//! assert_eq!(
//!     life,
//!     Rule {
//!         b: vec![3],
//!         s: vec![2, 3]
//!     }
//! )
//! ```

mod error;
mod hex;
mod isotropic;
mod lifelike;
mod traits;

pub use error::ParseRuleError;
pub use hex::Hex;
pub use isotropic::Isotropic;
pub use lifelike::Lifelike;
pub use traits::{Neighborhood, ParseBSRules};
