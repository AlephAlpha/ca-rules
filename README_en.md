# CA rules parsers

[![Travis (.org)](https://img.shields.io/travis/AlephAlpha/ca-rules)](https://travis-ci.org/AlephAlpha/ca-rules) [![Crates.io](https://img.shields.io/crates/v/ca-rules)](https://crates.io/crates/ca-rules) [![Docs.rs](https://docs.rs/ca-rules/badge.svg)](https://docs.rs/ca-rules/) [![中文](https://img.shields.io/badge/readme-%E4%B8%AD%E6%96%87-brightgreen)](README.md)

Parsing rule strings of life-like and other cellular automata.

Currently the following rules are supported:

* [Totalistic Life-like](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton),
  e.g., `B3/S23`.
* [Isotropic non-totalistic Life-like](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton),
  e.g., `B2ci3ai4c8/S02ae3eijkq4iz5ar6i7e`.
* [Totalistic Hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood),
  e.g., `B2/S34H`.
* [Isotropic non-totalistic Hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood),
  e.g., `B2o3-o4m/S12m3o4m5H`.
* [von Neumann neighbourhood](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood),
  e.g., `B2/S013V`.
* The corresponding [Generations rules](http://www.conwaylife.com/wiki/Generations)
of the above rules, e.g., `3457/357/5`.

For non-Generations rules, both [B/S notation](http://www.conwaylife.com/wiki/Rulestring#B.2FS_notation)
(`B3/S23`) and [S/B notation](http://www.conwaylife.com/wiki/Rulestring#S.2FB_notation)
(`23/3`) are supported.

For Generations rules, three different notations are supported:

* B/S notation (`B357/S3457/C5`)
* The notation used by [Golly](http://golly.sourceforge.net/Help/Algorithms/Generations.html) (`3457/357/5`)
* The notation used by [Catagolue](https://catagolue.appspot.com/rules/generations) (`g5b357s3457`)

Please refer to [Life Wiki](www.conwaylife.com/wiki/Rulestring) for detailed definitions and
notations of these rule strings.

## Example:

```rust
use ca_rules::ParseLife;

// Define a struct for your rule:
#[derive(Debug, Eq, PartialEq)]
struct Rule {
    b: Vec<u8>,
    s: Vec<u8>,
}

// Implement a parser trait for your rule:
// The choice of the trait depends on the type of rules you want to parse.
impl ParseLife for Rule {
    // Implement a function to construct the rule from b and s data:
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
        Rule { b, s }
    }
}

// Then you can parse a rule string:
let life = Rule::parse_rule("B3/S23").unwrap();
assert_eq!(
    life,
    Rule {
        b: vec![3],
        s: vec![2, 3],
    }
)
```

For details, please refer to the [doc](https://docs.rs/ca-rules/).
