# CA rules parsers

[![Travis (.org)](https://img.shields.io/travis/AlephAlpha/ca-rules)](https://travis-ci.org/AlephAlpha/ca-rules) [![Crates.io](https://img.shields.io/crates/v/ca-rules)](https://crates.io/crates/ca-rules) [![Docs.rs](https://docs.rs/ca-rules/badge.svg)](https://docs.rs/ca-rules/) [![English](https://img.shields.io/badge/readme-English-brightgreen)](README_en.md)

把 [rlifesrc](https://github.com/AlephAlpha/rlifesrc) 中读取元胞自动机的规则的部分拿出来，作为一个独立的 crate。

这只是一个 parser，没有别的功能。

目前支持以下规则：

* [Totalistic Life-like](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton)，如 `B3/S23`
* [Isotropic non-totalistic Life-like](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton)，如 `B35y/S1e2-ci3-a5i`
* [Totalistic Hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood)，如 `B2/S34H`
* [Isotropic non-totalistic Hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood)，如 `B2o3-o4m/S12m3o4m5H`
* [von Neumann 邻域](http://www.conwaylife.com/wiki/Von_Neumann_neighbourhood)，如 `B2/S013V`
* 以上规则对于的 [Generations](http://www.conwaylife.com/wiki/Generations) 规则，如 `3457/357/5`

对于非 Generations 的规则，支持 [B/S notation](http://www.conwaylife.com/wiki/Rulestring#B.2FS_notation)（如 `B3/S23`）和 [S/B notation](http://www.conwaylife.com/wiki/Rulestring#S.2FB_notation)（如 `3/23`）。

对于 Generations 规则，支持以下三种写法：

* B/S/C（`B357/S3457/C5`）
* [Golly](http://golly.sourceforge.net/Help/Algorithms/Generations.html) 的写法（`3457/357/5`）
* [Catagolue](https://catagolue.appspot.com/rules/generations) 的写法（`g5b357s3457`）


这些规则的具体定义和记号可见 [Life Wiki](www.conwaylife.com/wiki/Rulestring)。

## 用法

```rust
use ca_rules::ParseLife;

// 首先为规则定义一个结构体：
// 表示规则的方式并不唯一，可根据需要采用不同的定义。
#[derive(Debug, Eq, PartialEq)]
struct Rule {
    b: Vec<u8>,
    s: Vec<u8>,
}

// 为规则实现 parser trait：
// 根据规则的类型来选择相应的 trait。
impl ParseLife for Rule {
    // 定义一个函数，以从 b 和 s 的数据构造规则：
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
        Rule { b, s }
    }
}

// 然后就可以 parse 了：
let life = Rule::parse_rule("B3/S23").unwrap();
assert_eq!(
    life,
    Rule {
        b: vec![3],
        s: vec![2, 3],
    }
)
```

详见[文档](https://docs.rs/ca-rules/)。
