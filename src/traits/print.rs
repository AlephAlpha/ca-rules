use crate::util::Bs::{self, B, S};
use fixedbitset::FixedBitSet;

/// A trait for printing non-Generations rules.
pub trait PrintRule {
    /// The suffix of the rule string.
    const SUFFIX: Option<char>;

    /// Writing `b`/`s` data.
    fn write_bs(&self, string: &mut String, bs: Bs);

    /// Print the rule in B/S notation, e.g. `B3/S23`.
    fn to_string_bs(&self) -> String {
        let mut string = String::new();

        string.push('B');
        self.write_bs(&mut string, B);
        string.push('/');
        string.push('S');
        self.write_bs(&mut string, S);
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_uppercase());
        }
        string
    }

    /// Print the rule in S/B notation, e.g. `23/3`.
    fn to_string_sb(&self) -> String {
        let mut string = String::new();

        self.write_bs(&mut string, S);
        string.push('/');
        self.write_bs(&mut string, B);
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_uppercase());
        }
        string
    }

    /// Print the rule in S/B notation, e.g. `b3s23`.
    fn to_string_catagolue(&self) -> String {
        let mut string = String::new();

        string.push('b');
        self.write_bs(&mut string, B);
        string.push('s');
        self.write_bs(&mut string, S);
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_lowercase());
        }
        string
    }
}

/// A trait for printing Generations rules.
pub trait PrintGenRule {
    /// The suffix of the rule string.
    const SUFFIX: Option<char>;

    /// Writing `b`/`s` data.
    fn write_bs(&self, string: &mut String, bs: Bs);

    // The generation number.
    fn get_gen(&self) -> u32;

    /// Print the rule in B/S/G notation, e.g. `B3/S23/G3`.
    fn to_string_bsg(&self) -> String {
        let mut string = String::new();

        string.push('B');
        self.write_bs(&mut string, B);
        string.push('/');
        string.push('S');
        self.write_bs(&mut string, S);
        if self.get_gen() != 2 {
            string.push('/');
            string.push('G');
            string.push_str(&self.get_gen().to_string());
        }
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_uppercase());
        }
        string
    }

    /// Print the rule in S/B/G notation, e.g. `23/3/3`.
    fn to_string_sbg(&self) -> String {
        let mut string = String::new();

        self.write_bs(&mut string, S);
        string.push('/');
        self.write_bs(&mut string, B);
        if self.get_gen() != 2 {
            string.push('/');
            string.push_str(&self.get_gen().to_string());
        }
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_uppercase());
        }
        string
    }

    /// Print the rule in Catagolue's notation, e.g. `g3b3s23`.
    fn to_string_catagolue(&self) -> String {
        let mut string = String::new();

        if self.get_gen() != 2 {
            string.push('g');
            string.push_str(&self.get_gen().to_string());
        }
        string.push('b');
        self.write_bs(&mut string, B);
        string.push('s');
        self.write_bs(&mut string, S);
        if let Some(suffix) = Self::SUFFIX {
            string.push(suffix.to_ascii_lowercase());
        }
        string
    }
}

/// A trait for printing non-Generations rules with
/// [MAP string](http://golly.sourceforge.net/Help/Algorithms/QuickLife.html#map).
pub trait PrintMapRule {
    /// The data, in a [`FixedBitSet`].
    fn data(&self) -> &FixedBitSet;

    /// Print the rule in MAP string.
    fn to_string_map(&self) -> String {
        let mut string = String::new();

        string.push_str("MAP");

        let bytes: Vec<u8> = self
            .data()
            .as_slice()
            .iter()
            .map(|block| block.reverse_bits().to_be_bytes())
            .flatten()
            .collect();

        base64::encode_config_buf(&bytes, base64::STANDARD_NO_PAD, &mut string);
        string
    }
}
