use std::ascii::AsciiExt;
use std::char;

/// Provides a fast method for testing character membership of a purely ASCII set.
///
/// This is implemented as a bitset, and will therefore always use 16 bytes, no matter how many
/// characters the set contains.
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct AsciiSet {
    /// The bitmask representing characters 0 through 63. If `c <= 63` and `(lo_mask >> c) & 1 ==
    /// 1` then the ASCII codepoint `c` belongs to this set.
    pub lo_mask: u64,
    /// The bitmask representing characters 64 through 127.
    pub hi_mask: u64,
}

impl AsciiSet {
    #[inline(always)]
    fn contains(&self, c: u32) -> bool {
        let mask = if c < 64 {
            self.lo_mask
        } else if c < 128 {
            self.hi_mask
        } else {
            0
        };
        ((mask >> (c % 64)) & 1) == 1
    }

    /// Tests whether this set contains the char `c`.
    #[inline(always)]
    pub fn contains_char(&self, c: char) -> bool {
        self.contains(c as u32)
    }

    /// Tests whether this set contains the byte `c`.
    #[inline(always)]
    pub fn contains_byte(&self, c: u8) -> bool {
        self.contains(c as u32)
    }

    /// Adds a byte to this set.
    ///
    /// # Panics
    ///  - if `c` falls outside the ASCII range.
    pub fn insert_byte(&mut self, c: u8) {
        if c >= 128 {
            panic!("only ASCII chars allowed");
        }
        let mask = if c < 64 {
            &mut self.lo_mask
        } else {
            &mut self.hi_mask
        };
        *mask |= 1 << (c % 64);
    }

    /// Adds a char to this set.
    ///
    /// # Panics
    ///  - if `c` falls outside the ASCII range.
    pub fn insert_char(&mut self, c: char) {
        if !c.is_ascii() {
            panic!("only ASCII chars allowed");
        }
        self.insert_byte(c as u8);
    }

    /// Creates a new, empty, `AsciiSet`.
    pub fn new() -> AsciiSet {
        AsciiSet {
            lo_mask: 0,
            hi_mask: 0,
        }
    }

    /// Builds an `AsciiSet` as a union of ranges (which are inclusive).
    ///
    /// # Panics
    ///  - if any of the ranges overlap anything outside the ASCII range.
    ///
    /// # Examples
    /// ```
    /// use ascii_set::AsciiSet;
    /// let a = AsciiSet::from_ranges(vec![('a', 'e'), ('A', 'E')]);
    /// assert!(a.contains_char('a'));
    /// assert!(a.contains_char('b'));
    /// assert!(a.contains_char('e'));
    /// assert!(!a.contains_char('f'));
    /// ```
    pub fn from_ranges<I>(iter: I) -> AsciiSet
            where I: IntoIterator<Item=(char, char)> {
        let mut ret = AsciiSet::new();
        for range in iter {
            if range.1 as u32 >= 128 {
                panic!("only ASCII chars allowed");
            }
            for c in (range.0 as u32) .. (range.1 as u32 + 1) {
                ret.insert_byte(c as u8);
            }
        }
        ret
    }

    /// Builds the `AsciiSet` consisting of all characters for which `f` returns `true`.
    ///
    /// # Examples
    /// ```
    /// use ascii_set::AsciiSet;
    /// assert_eq!(
    ///     AsciiSet::from_ranges(vec![('a', 'z'), ('A', 'Z')]),
    ///     AsciiSet::from_fn(|c| c.is_alphabetic()));
    pub fn from_fn<F>(mut f: F) -> AsciiSet
            where F: FnMut(char) -> bool {
        let mut ret = AsciiSet::new();
        for c in 0..128 {
            if f((char::from_u32(c as u32)).unwrap()) {
                ret.insert_byte(c);
            }
        }
        ret
    }

    /// Builds the `AsciiSet` consisting of all characters yielded by `iter`.
    ///
    /// # Panics
    ///  - if `iter` yields any non-ASCII characters.
    ///
    /// # Examples
    /// ```
    /// use ascii_set::AsciiSet;
    /// assert_eq!(
    ///     AsciiSet::from_ranges(vec![('a', 'z')]),
    ///     AsciiSet::from_chars("abcdefghijklmnopqrstuvwxyz".chars()));
    /// ```
    pub fn from_chars<I>(iter: I) -> AsciiSet
            where I: IntoIterator<Item=char> {
        let mut ret = AsciiSet::new();
        for c in iter {
            ret.insert_char(c);
        }
        ret
    }

    /// Returns the union of this set and `other`.
    /// # Examples
    /// ```
    /// use ascii_set::AsciiSet;
    /// assert_eq!(
    ///     AsciiSet::letters(),
    ///     AsciiSet::upper_case_letters().union(&AsciiSet::lower_case_letters()));
    /// ```
    pub fn union(&self, other: &AsciiSet) -> AsciiSet {
        AsciiSet {
            lo_mask: self.lo_mask | other.lo_mask,
            hi_mask: self.hi_mask | other.hi_mask,
        }
    }

    /// Returns the intersection of this set and `other`.
    ///
    /// # Examples
    /// ```
    /// use ascii_set::AsciiSet;
    /// assert_eq!(
    ///     AsciiSet::lower_case_letters(),
    ///     AsciiSet::letters().intersection(&AsciiSet::lower_case_letters()));
    /// ```
    pub fn intersection(&self, other: &AsciiSet) -> AsciiSet {
        AsciiSet {
            lo_mask: self.lo_mask & other.lo_mask,
            hi_mask: self.hi_mask & other.hi_mask,
        }
    }

    /// Returns the set of chars in `self` but not `other`.
    ///
    /// # Examples
    /// ```
    /// use ascii_set::AsciiSet;
    /// assert_eq!(
    ///     AsciiSet::lower_case_letters(),
    ///     AsciiSet::letters().difference(&AsciiSet::upper_case_letters()));
    /// ```
    pub fn difference(&self, other: &AsciiSet) -> AsciiSet {
        self.intersection(&other.complement())
    }

    /// Returns the set of all ASCII chars not in `self`.
    pub fn complement(&self) -> AsciiSet {
        AsciiSet {
            lo_mask: !self.lo_mask,
            hi_mask: !self.hi_mask,
        }
    }

    /// Returns the set of all lower case letters.
    pub fn lower_case_letters() -> AsciiSet {
        AsciiSet {
            lo_mask: 0,
            hi_mask: 0b0000011111111111111111111111111000000000000000000000000000000000,
        }
    }

    /// Returns the set of all upper case letters.
    pub fn upper_case_letters() -> AsciiSet {
        AsciiSet {
            lo_mask: 0,
            hi_mask: 0b0000000000000000000000000000000000000111111111111111111111111110,
        }
    }

    /// Returns the set of all letters.
    pub fn letters() -> AsciiSet {
        AsciiSet::lower_case_letters().union(&AsciiSet::upper_case_letters())
    }

    /// Returns the set of all digits.
    pub fn digits() -> AsciiSet {
        AsciiSet {
            lo_mask: 0b0000001111111111000000000000000000000000000000000000000000000000,
            hi_mask: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use ::AsciiSet;

    #[test]
    fn builtin_ranges() {
        assert_eq!(AsciiSet::lower_case_letters(), AsciiSet::from_ranges(vec![('a', 'z')]));
        assert_eq!(AsciiSet::upper_case_letters(), AsciiSet::from_ranges(vec![('A', 'Z')]));
        assert_eq!(AsciiSet::letters(), AsciiSet::from_ranges(vec![('A', 'Z'), ('a', 'z')]));
        assert_eq!(AsciiSet::digits(), AsciiSet::from_ranges(vec![('0', '9')]));
    }
}

