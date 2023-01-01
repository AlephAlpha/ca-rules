#![macro_use]

/// A macro to define a helper struct that represents the rule.
macro_rules! rule_struct {
    ($name: ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct $name {
            b: Vec<u8>,
            s: Vec<u8>,
        }

        impl $name {
            /// Construct the struct from `b` / `s` data.
            fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
                $name { b, s }
            }

            /// Construct the Generations struct from `b` / `s` data and the number of states.
            fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Gen<Self> {
                Gen {
                    rule: $name { b, s },
                    gen,
                }
            }

            /// A parser for numbers.
            fn parse_num<I>(chars: &mut std::iter::Peekable<I>) -> Result<usize, ParseRuleError>
            where
                I: Iterator<Item = char>,
            {
                let mut n: usize = 0;
                if chars.peek().is_none() || !chars.peek().unwrap().is_digit(10) {
                    return Err(ParseRuleError::MissingNumber);
                }
                while let Some(&c) = chars.peek() {
                    match c {
                        c if c.is_digit(10) => {
                            chars.next();
                            n = n
                                .checked_mul(10)
                                .ok_or(ParseRuleError::GenOverflow)?
                                .checked_add(c.to_digit(10).unwrap() as usize)
                                .ok_or(ParseRuleError::GenOverflow)?;
                        }
                        _ => return Ok(n),
                    }
                }
                Ok(n)
            }
        }
    };
}

/// A macro to define a function to parse the helper struct.
macro_rules! parse_rule {
    ($($suffix: expr)?) => {
        /// A parser for the struct.
        fn parse_rule(input: &str) -> Result<Self, ParseRuleError> {
            let mut chars = input.chars().peekable();
            let (b, s);

            match chars.peek() {
                Some('B') | Some('b') => {
                    // Rule strings using B/S notation
                    chars.next();
                    b = Self::parse_bs(&mut chars)?;
                    if chars.peek() == Some(&'/') {
                        chars.next();
                    }
                    match chars.next() {
                        Some('S') | Some('s') => (),
                        _ => return Err(ParseRuleError::Missing('S')),
                    }
                    s = Self::parse_bs(&mut chars)?;
                }
                _ => {
                    // Rule strings using S/B notation
                    s = Self::parse_bs(&mut chars)?;
                    match chars.next() {
                        Some('/') => (),
                        _ => return Err(ParseRuleError::Missing('/')),
                    }
                    b = Self::parse_bs(&mut chars)?;
                }
            }

            $(
                // Suffix
                if let Some(c) = chars.next() {
                    if $suffix.to_lowercase().chain($suffix.to_uppercase()).all(|s| s != c) {
                        return Err(ParseRuleError::Missing($suffix));
                    }
                } else {
                    return Err(ParseRuleError::Missing($suffix));
                }
            )?

            match chars.next() {
                None => Ok(Self::from_bs(b, s)),
                _ => Err(ParseRuleError::ExtraJunk),
            }
        }

        /// A parser for the Generations struct.
        fn parse_rule_gen(input: &str) -> Result<Gen<Self>, ParseRuleError> {
            let mut chars = input.chars().peekable();
            let (b, s);
            let mut gen = 2;

            match chars.peek() {
                // Rule strings using B/S/G notation
                Some('B') | Some('b') => {
                    chars.next();
                    b = Self::parse_bs(&mut chars)?;
                    if chars.peek() == Some(&'/') {
                        chars.next();
                    }
                    match chars.next() {
                        Some('S') | Some('s') => (),
                        _ => return Err(ParseRuleError::Missing('S')),
                    }
                    s = Self::parse_bs(&mut chars)?;
                    match chars.peek() {
                        Some('/') => {
                            chars.next();
                            match chars.peek() {
                                Some('C') | Some('c') | Some('G') | Some('g') => {
                                    chars.next();
                                }
                                _ => (),
                            }
                            gen = Self::parse_num(&mut chars)?;
                        }
                        Some('C') | Some('c') | Some('G') | Some('g') => {
                            chars.next();
                            gen = Self::parse_num(&mut chars)?;
                        }
                        _ => (),
                    }
                }

                // Rule strings using G/B/S notation
                Some('C') | Some('c') | Some('G') | Some('g') => {
                    chars.next();
                    gen = Self::parse_num(&mut chars)?;
                    if chars.peek() == Some(&'/') {
                        chars.next();
                    }
                    match chars.next() {
                        Some('B') | Some('b') => (),
                        _ => return Err(ParseRuleError::Missing('B')),
                    }
                    b = Self::parse_bs(&mut chars)?;
                    if chars.peek() == Some(&'/') {
                        chars.next();
                    }
                    match chars.next() {
                        Some('S') | Some('s') => (),
                        _ => return Err(ParseRuleError::Missing('S')),
                    }
                    s = Self::parse_bs(&mut chars)?;
                }

                // Rule strings using S/B/G notation
                _ => {
                    s = Self::parse_bs(&mut chars)?;
                    match chars.next() {
                        Some('/') => (),
                        _ => return Err(ParseRuleError::Missing('/')),
                    }
                    b = Self::parse_bs(&mut chars)?;
                    if chars.peek() == Some(&'/') {
                        chars.next();
                        gen = Self::parse_num(&mut chars)?;
                    }
                }
            }

            $(
                // Suffix
                if let Some(c) = chars.next() {
                    if $suffix.to_lowercase().chain($suffix.to_uppercase()).all(|s| s != c) {
                        return Err(ParseRuleError::Missing($suffix));
                    }
                } else {
                    return Err(ParseRuleError::Missing($suffix));
                }
            )?

            if gen < 2 {
                Err(ParseRuleError::GenLessThan2)
            } else {
                match chars.next() {
                    None => Ok(Self::from_bsg(b, s, gen)),
                    _ => Err(ParseRuleError::ExtraJunk),
                }
            }
        }
    };
}

/// A macro to define a function to parse `b` / `s` data.
macro_rules! parse_bs {
    ($n: expr) => {
        /// A parser for `b` / `s` data.
        fn parse_bs<I>(chars: &mut std::iter::Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
        where
            I: Iterator<Item = char>,
        {
            let mut bs = Vec::new();

            while let Some(&c) = chars.peek() {
                match c {
                    c if c.is_digit($n + 1) => {
                        chars.next();
                        bs.push(c.to_digit($n + 1).unwrap() as u8);
                    }
                    _ => break,
                }
            }

            bs.sort_unstable();
            Ok(bs)
        }
    };

    { $($count: expr => { $($key: expr => $value: expr),* $(,)? }),*  $(,)? } => {
        /// A parser for `b` / `s` data.
        fn parse_bs<I>(chars: &mut std::iter::Peekable<I>) -> Result<Vec<u8>, ParseRuleError>
        where
            I: Iterator<Item = char>,
        {
            let mut bs = Vec::new();

            while let Some(&c) = chars.peek() {
                match c {
                    $(
                        $count => {
                            chars.next();
                            let all_keys = vec![$($key),*];
                            let keys = match chars.peek() {
                                Some('-') => {
                                    chars.next();
                                    let mut keys = Vec::new();
                                    while let Some(&c) = chars.peek() {
                                        if all_keys.contains(&c) {
                                            chars.next();
                                            keys.push(c);
                                        } else {
                                            break;
                                        }
                                    }
                                    all_keys.into_iter().filter(|c| !keys.contains(c)).collect()
                                }
                                Some(c) if all_keys.contains(&c) => {
                                    let mut keys = Vec::new();
                                    while let Some(&c) = chars.peek() {
                                        if all_keys.contains(&c) {
                                            chars.next();
                                            keys.push(c);
                                        } else {
                                            break;
                                        }
                                    }
                                    keys
                                }
                                Some(_) => {
                                    all_keys
                                }
                                None => all_keys
                            };
                            for &c in keys.iter() {
                                match c {
                                    $(
                                        $key => bs.extend_from_slice(&($value)),
                                    )*
                                    _ => unreachable!(),
                                }
                            }
                        }
                    ),*
                    _ => break,
                }
            }

            bs.sort_unstable();
            Ok(bs)
        }
    };
}

/// A macro to define a function to parse MAP strings.
macro_rules! parse_rule_map {
    ($n: expr) => {
        /// A parser for the struct that parses MAP strings.
        fn parse_rule_map(input: &str) -> Result<Self, ParseRuleError> {
            use base64::{
                alphabet::STANDARD,
                engine::{
                    fast_portable::{FastPortable, FastPortableConfig},
                    DecodePaddingMode,
                },
            };

            const CENTER_MARK: usize = 1 << ($n / 2);
            const RIGHT_MARK: usize = CENTER_MARK - 1;
            const LEFT_MARK: usize = RIGHT_MARK << ($n / 2 + 1);
            const ENGINE_CONFIG: FastPortableConfig =
                FastPortableConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent);
            const ENGINE: FastPortable = FastPortable::from(&STANDARD, ENGINE_CONFIG);

            if !input.starts_with("MAP") {
                return Err(ParseRuleError::NotMapRule);
            }
            let bytes = base64::decode_engine(&input[3..], &ENGINE)
                .map_err(|_| ParseRuleError::Base64Error)?;
            if bytes.len() * 8 != 2 << $n {
                return Err(ParseRuleError::InvalidLength);
            }
            let mut b = Vec::new();
            let mut s = Vec::new();
            for (i, x) in bytes.iter().map(|x| x.reverse_bits()).enumerate() {
                for j in 0..8 {
                    if x & (1 << j) != 0 {
                        let k = i * 8 + j;
                        let n = ((k & LEFT_MARK) >> 1 | (k & RIGHT_MARK)) as u8;
                        if k & CENTER_MARK == 0 {
                            b.push(n);
                        } else {
                            s.push(n);
                        }
                    }
                }
            }
            Ok(Self::from_bs(b, s))
        }

        /// A parser for the Generations struct that parses MAP strings.
        fn parse_rule_gen_map(input: &str) -> Result<Gen<Self>, ParseRuleError> {
            use base64::{
                alphabet::STANDARD,
                engine::{
                    fast_portable::{FastPortable, FastPortableConfig},
                    DecodePaddingMode,
                },
            };

            const CENTER_MARK: usize = 1 << ($n / 2);
            const RIGHT_MARK: usize = CENTER_MARK - 1;
            const LEFT_MARK: usize = RIGHT_MARK << ($n / 2 + 1);
            const ENGINE_CONFIG: FastPortableConfig =
                FastPortableConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent);
            const ENGINE: FastPortable = FastPortable::from(&STANDARD, ENGINE_CONFIG);

            let mut gen = 2;
            let mut slash = input.len();
            if !input.starts_with("MAP") {
                return Err(ParseRuleError::NotMapRule);
            }
            if let Some(n) = input.rfind('/') {
                if (n - 3) * 6 >= 2 << $n {
                    slash = n;
                    let mut chars = input[n + 1..].chars().peekable();
                    if chars.peek().is_some() {
                        gen = Self::parse_num(&mut chars)?;
                        if chars.next().is_some() {
                            return Err(ParseRuleError::ExtraJunk);
                        }
                    }
                }
            }
            let bytes = base64::decode_engine(&input[3..slash], &ENGINE)
                .map_err(|_| ParseRuleError::Base64Error)?;
            if bytes.len() * 8 != 2 << $n {
                return Err(ParseRuleError::InvalidLength);
            }
            let mut b = Vec::new();
            let mut s = Vec::new();
            for (i, x) in bytes.iter().map(|x| x.reverse_bits()).enumerate() {
                for j in 0..8 {
                    if x & (1 << j) != 0 {
                        let k = i * 8 + j;
                        let n = ((k & LEFT_MARK) >> 1 | (k & RIGHT_MARK)) as u8;
                        if k & CENTER_MARK == 0 {
                            b.push(n);
                        } else {
                            s.push(n);
                        }
                    }
                }
            }
            Ok(Self::from_bsg(b, s, gen))
        }
    };
}

/// A macro for implementing traits for helper structs.
///
/// `$f` is a function that converts the `b` / `s` data of the struct to those of the trait.
/// `$n` is the upper bound of `b` / `s` data of the struct.
macro_rules! impl_parser {
    (
        ($trait_name: ident, $trait_name_gen: ident) for $struct_name: ident,
        $f: expr,
        $n: expr $(,)?
    ) => {
        impl $trait_name for $struct_name {
            fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
                let mut new_b = Vec::new();
                let mut new_s = Vec::new();
                let f = $f;
                for i in 0_u8..=$n {
                    let j = f(i);
                    if b.contains(&j) {
                        new_b.push(i);
                    }
                    if s.contains(&j) {
                        new_s.push(i);
                    }
                }
                $struct_name::from_bs(new_b, new_s)
            }
        }

        impl $trait_name_gen for Gen<$struct_name> {
            fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Self {
                let mut new_b = Vec::new();
                let mut new_s = Vec::new();
                let f = $f;
                for i in 0_u8..=$n {
                    let j = f(i);
                    if b.contains(&j) {
                        new_b.push(i);
                    }
                    if s.contains(&j) {
                        new_s.push(i);
                    }
                }
                $struct_name::from_bsg(new_b, new_s, gen)
            }
        }
    };
}
