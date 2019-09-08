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
            fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
                $name { b, s }
            }

            fn from_bsg(b: Vec<u8>, s: Vec<u8>, gen: usize) -> Gen<Self> {
                Gen {
                    rule: $name { b, s },
                    gen,
                }
            }
        }
    };
}

/// A macro to define a function to parse the helper struct.
macro_rules! parse_rule {
    ($($suffix: expr)?) => {
        /// A parser for the struct.
        fn parse_rule(input: &str) -> Result<Self, ParseRuleError>
        {
            let mut chars = input.chars().peekable();
            let (b, s);

            match chars.peek() {
                Some('B') | Some('b') => {
                    // Rule strings using B/S notation
                    chars.next();
                    b = Self::parse_bs(&mut chars)?;
                    if let Some('/') = chars.peek() {
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

        /// A parser for the struct.
        fn parse_rule_gen(input: &str) -> Result<Gen<Self>, ParseRuleError>
        {
            let mut chars = input.chars().peekable();
            let (b, s);
            let mut gen = 2;

            match chars.peek() {
                // Rule strings using B/S/C notation
                Some('B') | Some('b') => {
                    chars.next();
                    b = Self::parse_bs(&mut chars)?;
                    if let Some('/') = chars.peek() {
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

                // Rule strings using C/B/S notation
                Some('C') | Some('c') | Some('G') | Some('g') => {
                    chars.next();
                    gen = Self::parse_num(&mut chars)?;
                    if let Some('/') = chars.peek() {
                        chars.next();
                    }
                    match chars.next() {
                        Some('B') | Some('b') => (),
                        _ => return Err(ParseRuleError::Missing('B')),
                    }
                    b = Self::parse_bs(&mut chars)?;
                    if let Some('/') = chars.peek() {
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
                    if let Some('/') = chars.peek() {
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
            } else {match chars.next() {
                    None => Ok(Self::from_bsg(b, s, gen)),
                    _ => Err(ParseRuleError::ExtraJunk),
                }
            }
        }

        fn parse_num<I>(chars: &mut std::iter::Peekable<I>) -> Result<usize, ParseRuleError>
        where
            I: Iterator<Item = char>,
        {
            let mut n = 0;
            if chars.peek().is_none() {
                return Err(ParseRuleError::MissingNumber);
            }
            while let Some(&c) = chars.peek() {
                match c {
                    c if c.is_digit(10) => {
                        chars.next();
                        n *= 10;
                        n += c.to_digit(10).unwrap() as usize;
                    }
                    _ => return Ok(n),
                }
            }
            Ok(n)
        }
    };
}

/// A macro to define a function to parse `b` or `s` data.
macro_rules! parse_bs {
    ($n: expr) => {
        /// A parser for `b` or `s` data.
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

            bs.sort();
            Ok(bs)
        }
    };

    { $($count: expr => { $($key: expr => $value: expr),* $(,)? }),*  $(,)? } => {
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

            bs.sort();
            Ok(bs)
        }
    };
}
