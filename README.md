# CA rules parsers

把 [rlifesrc](https://github.com/AlephAlpha/rlifesrc) 中读取元胞自动机的规则的部分拿出来，作为一个独立的 crate。

目前支持以下规则：

* [Totalistic Life-like](http://www.conwaylife.com/wiki/Totalistic_Life-like_cellular_automaton)，如 `B3/S23`
* [Isotropic non-totalistic Life-like](http://www.conwaylife.com/wiki/Isotropic_non-totalistic_Life-like_cellular_automaton)，如 `B2ci3ai4c8/S02ae3eijkq4iz5ar6i7e`
* [Totalistic Hexagonal](http://www.conwaylife.com/wiki/Hexagonal_neighbourhood)，如 `B2/S34H`

这些规则的具体定义和记号可见 [Life Wiki](www.conwaylife.com/wiki/Rulestring)。

支持 [B/S notation](http://www.conwaylife.com/wiki/Rulestring#B.2FS_notation)（如 `B3/S23`）和 [S/B notaion](http://www.conwaylife.com/wiki/Rulestring#S.2FB_notation)（如 `3/23`）。

这只是一个 parser，没有别的功能。

## 用法

```rust
use ca_rules::{Lifelike, ParseBSRules};

// 首先需要为规则定义一个结构体：
// 表示规则的方式并不唯一，可根据需要采用不同的定义。
#[derive(Debug, Eq, PartialEq)]
struct Rule {
    b: Vec<u8>,
    s: Vec<u8>,
}

// 为规则实现 ParseBSRules trait：
impl ParseBSRules for Rule {
    // 制定规则的领域类型：
    type Neighborhood = Lifelike;

    // 定义一个函数，以从 b 和 s 的数据构造规则：
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
        Rule { b, s }
    }
}

// 然后就可以 parse 了：
let life = Rule::parse_rule(&"B3/S23").unwrap();
assert_eq!(
    life,
    Rule {
        b: vec![3],
        s: vec![2, 3]
    }
)
```