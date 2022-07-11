use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SortedLetters {
    letters: [u8; 26],
}

impl SortedLetters {
    pub fn from_word(letters: &str) -> Self {
        let mut sorted = [0; 26];
        for ch in letters.chars() {
            if ch.is_ascii_alphabetic() {
                sorted[ch.to_ascii_lowercase() as usize - 'a' as usize] += 1;
            }
        }
        Self { letters: sorted }
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        self.letters
            .iter()
            .zip(other.letters.iter())
            .all(|(a, b)| a >= b)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.letters
            .iter()
            .zip(other.letters.iter())
            .all(|(a, b)| a <= b)
    }

    pub fn minus(&self, other: &Self) -> Option<Self> {
        let mut sorted = self.letters;
        for (i, a) in sorted.iter_mut().enumerate() {
            *a = a.checked_sub(other.letters[i])?;
        }
        Some(Self { letters: sorted })
    }
}

impl TryFrom<&str> for SortedLetters {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self::from_word(s))
    }
}

impl TryFrom<Option<&str>> for SortedLetters {
    type Error = ();
    fn try_from(s: Option<&str>) -> Result<Self, Self::Error> {
        if let Some(s) = s {
            Self::try_from(s)
        } else {
            Err(())
        }
    }
}

impl Display for SortedLetters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &count) in self.letters.iter().enumerate() {
            for _ in 0..count {
                write!(f, "{}", (i as u8 + b'a') as char)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn test_new() {
        let mut expected = [0; 26];
        expected[0] = 2;
        expected[1] = 2;
        expected[2] = 1;
        let sorted = SortedLetters::from_word("abcab");
        check!(sorted.letters == expected);
    }

    #[test]
    fn test_is_superset() {
        let sorted = SortedLetters::from_word("cabled");
        check!(sorted.is_superset(&SortedLetters::from_word("cabled")));
        check!(sorted.is_superset(&SortedLetters::from_word("able")));
        check!(sorted.is_superset(&SortedLetters::from_word("laced")));
        check!(sorted.is_superset(&SortedLetters::from_word("l")));
        check!(sorted.is_superset(&SortedLetters::from_word("")));
        check!(!sorted.is_superset(&SortedLetters::from_word("call")));
        check!(!sorted.is_superset(&SortedLetters::from_word("black")));
    }

    #[test]
    fn test_subtract() {
        let a = SortedLetters::from_word("deafened");
        let b = SortedLetters::from_word("faded");
        let c = a.minus(&b).unwrap();
        check!(c == SortedLetters::from_word("een"));
    }
}
