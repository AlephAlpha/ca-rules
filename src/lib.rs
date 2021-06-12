//! Parsing rule strings of life-like and other cellular automata.
//!
//! Currently the following rules are supported:
//!
//! * [Totalistic Life-like](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton),
//!   e.g., `B3/S23`.
//! * [Isotropic non-totalistic Life-like](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton),
//!   e.g., `B2ci3ai4c8/S02ae3eijkq4iz5ar6i7e`.
//! * [Non-isotropic Life-like](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton),
//!   e.g., `MAPARYXfhZofugWaH7oaIDogBZofuhogOiAaIDogIAAgAAWaH7oaIDogGiA6ICAAIAAaIDogIAAgACAAIAAAAAAAA`.
//! * [Totalistic Hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood),
//!   e.g., `B2/S34H`.
//! * [Isotropic non-totalistic Hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood),
//!   e.g., `B2o3-o4m/S12m3o4m5H`.
//! * [Non-isotropic Hexagonal](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton),
//!   e.g., `MAPFgFoF2gXgH5oF4B+gH4A6AH`.
//! * [von Neumann neighborhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood),
//!   e.g., `B2/S013V`.
//! * [Non-isotropic von Neumann](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton),
//!   e.g., `MAPHmlphg`.
//! * The corresponding [Generations rules](http://www.conwaylife.com/wiki/Generations)
//! of the above rules, e.g., `3457/357/5`.
//!
//! For non-Generations rules, four different notations are supported:
//! * [B/S notation](http://www.conwaylife.com/wiki/Rulestring#B.2FS_notation) (`B3/S23`)
//! * [S/B notation](http://www.conwaylife.com/wiki/Rulestring#S.2FB_notation) (`23/3`)
//! * [MAP strings](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
//!   for [non-isotropic rules](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
//!   (`MAPARYXfhZofugWaH7oaIDogBZofuhogOiAaIDogIAAgAAWaH7oaIDogGiA6ICAAIAAaIDogIAAgACAAIAAAAAAAA`)
//!
//! For Generations rules, four different notations are supported:
//!
//! * B/S notation (`B357/S3457/C5`)
//! * The notation used by [Golly](http://golly.sourceforge.net/Help/Algorithms/Generations.html) (`3457/357/5`)
//! * The notation used by [Catagolue](https://catagolue.appspot.com/rules/generations) (`g5b357s3457`)
//! * [MAP strings](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
//!   for [non-isotropic rules](http://www.conwaylife.com/wiki/Non-isotropic_Life-like_cellular_automaton)
//!   (`MAPARYBFxZpF38WaRd/aZZ//hZpF39pln/+aZZ//pZp/ukWaRd/aZZ//mmWf/6Waf7paZZ//pZp/umWaf7paZbplg/5`)
//!
//! Please refer to [Life Wiki](http://www.conwaylife.com/wiki/Rulestring) for detailed definitions and
//! notations of these rule strings.
//!
//! # Example:
//! ```
//! use ca_rules::ParseLife;
//!
//! // Define a struct for your rule:
//! #[derive(Debug, Eq, PartialEq)]
//! struct Rule {
//!     b: Vec<u8>,
//!     s: Vec<u8>,
//! }
//!
//! // Implement a parser trait for your rule:
//! // The choice of the trait depends on the type of rules you want to parse.
//! impl ParseLife for Rule {
//!     // Implement a function to construct the rule from b and s data:
//!     fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
//!         Rule { b, s }
//!     }
//! }
//!
//! // Then you can parse a rule string:
//! let life = Rule::parse_rule("B3/S23").unwrap();
//! assert_eq!(
//!     life,
//!     Rule {
//!         b: vec![3],
//!         s: vec![2, 3],
//!     }
//! )
//! ```

mod error;
mod macros;
pub mod new_rules;
mod rules;
pub mod traits;
mod util;

pub use error::{ConvertRuleError, ParseRuleError};
pub use fixedbitset;
pub use new_rules::*;
pub use rules::*;
pub use traits::*;
pub use util::Bs;

#[cfg(test)]
mod test {
    use super::{ParseNtLife, ParseRuleError};
    use base64::decode;

    #[derive(Debug, Eq, PartialEq)]
    struct Rule {
        b: Vec<u8>,
        s: Vec<u8>,
    }

    impl ParseNtLife for Rule {
        fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
            Rule { b, s }
        }
    }

    #[test]
    fn base64() -> Result<(), ParseRuleError> {
        let s = "MAPARYXfhZofugWaH7oaIDogBZofuhogOiAaIDogIAAgAAWaH7oaIDogGiA6ICAAIAAaIDogIAAgACAAIAAAAAAAA";
        let bytes = decode(&s[3..]).map_err(|_| ParseRuleError::Base64Error)?;
        assert_eq!(bytes.len(), 0x200 / 8);
        let mut b = Vec::new();
        let mut s = Vec::new();
        for (i, x) in bytes.iter().map(|x| x.reverse_bits()).enumerate() {
            for j in 0..8 {
                if x & (1 << j) != 0 {
                    let k = i * 8 + j;
                    let n = ((k & 0x1e0) >> 1 | (k & 0x0f)) as u8;
                    if k & 0x10 == 0 {
                        b.push(n);
                    } else {
                        s.push(n);
                    }
                }
            }
        }
        assert_eq!(Rule { b, s }, Rule::parse_rule("B3/S23")?);
        Ok(())
    }
}
