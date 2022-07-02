mod sorted_letters;
mod length_range;
mod filter;

pub use sorted_letters::SortedLetters;
pub use length_range::LengthRange;
pub use filter::{Filter, FilterBuilder};

use std::collections::HashMap;

/// A lexicon is a collection of words plus popularity data.
pub struct Lexicon<'a> {
    // Each entry is a word with some extra data to help with searching.
    entries: Vec<Entry<'a>>,
    // Index of entries by their sorted letters.  This greatly speeds
    // up the inner loop of SearchService::anagram_breakdowns().
    from_sorted: HashMap<SortedLetters, Vec<usize>>,

    /// The maximum rank for an entry to be considered "very popular", rather
    /// than "less popular".
    popular_threshold: usize,
}

impl<'a> Lexicon<'a> {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn with_popular_words(
        mut self,
        popular_words: impl IntoIterator<Item = &'a str>,
        threshold: usize,
    ) -> Self {
        let popularity = popular_words
            .into_iter()
            .enumerate()
            .map(|(rank, word)| (word, rank))
            .collect::<HashMap<_, _>>();

        for entry in &mut self.entries {
            entry.rank = popularity.get(&entry.word).copied();
        }
        self.popular_threshold = threshold;

        self
    }

    pub fn rate(&self, entry: &Entry<'_>) -> Popularity {
        match entry.rank {
            Some(rank) => {
                if rank < self.popular_threshold {
                    Popularity::High
                } else {
                    Popularity::Medium
                }
            }
            None => Popularity::Low,
        }
    }
}

impl Lexicon<'static> {
    /// Load the default lexicon.
    pub fn load() -> Self {
        const WORDS: &str = include_str!("../lexicon/twl06.txt");
        const POPULAR_WORDS: &str = include_str!("../lexicon/tv2006.txt");

        Lexicon::from_iter(WORDS.lines()).with_popular_words(POPULAR_WORDS.lines(), 10000)
    }
}

impl<'a> FromIterator<&'a str> for Lexicon<'a> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut entries = Vec::new();
        let mut from_sorted: HashMap<SortedLetters, Vec<usize>> = HashMap::new();
        for (idx, word) in iter.into_iter().enumerate() {
            let entry = Entry::new(word);
            from_sorted.entry(entry.sorted).or_default().push(idx);
            entries.push(entry);
        }
        Lexicon {
            from_sorted,
            entries,
            popular_threshold: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Entry<'a> {
    word: &'a str,
    rank: Option<usize>,
    mask: LetterMask,
    sorted: SortedLetters,
    len: usize,
    one_word: bool,
}

pub struct Anagrams<'a> {
    entries: &'a [Entry<'a>],
    indexes: Vec<usize>,
    idx: usize,
}

impl<'a> Iterator for Anagrams<'a> {
    type Item = &'a Entry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.indexes.len() {
            return None;
        }
        let idx = self.indexes[self.idx];
        self.idx += 1;
        Some(&self.entries[idx])
    }
}

impl<'a> Lexicon<'a> {
    pub fn entries(&'a self) -> impl Iterator<Item = &'a Entry<'a>> {
        self.entries.iter()
    }

    pub fn solve_anagram(&'a self, letters: &SortedLetters) -> Anagrams<'a> {
        let indexes = self.from_sorted.get(letters).cloned().unwrap_or_default();
        Anagrams {
            entries: &self.entries,
            indexes,
            idx: 0,
        }
    }

    #[allow(dead_code)]
    pub fn matching_words(&'a self, filter: &'a Filter) -> impl Iterator<Item = &'a str> + 'a {
        self.entries()
            .filter(|entry| filter.matches(entry))
            .map(|e| e.word)
    }
}

impl<'a> Entry<'a> {
    pub fn new(word: &'a str) -> Self {
        let one_word = word.chars().all(|ch| ch.is_ascii_alphabetic());
        let letters = word
            .chars()
            .filter(char::is_ascii_alphabetic)
            .collect::<String>();
        let mask = LetterMask::new(&letters);
        let sorted = SortedLetters::from_word(&letters);
        let len = word.chars().filter(|ch| ch.is_alphabetic()).count();
        Self {
            word,
            rank: None,
            mask,
            sorted,
            len,
            one_word,
        }
    }

    /// Returns the number of alphabetic characters in the word.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn num_letters(&self) -> usize {
        self.len
    }

    pub fn is_single_word(&self) -> bool {
        self.one_word
    }

    pub fn word(&self) -> &str {
        self.word
    }

    pub fn without_letters_in(&self, sorted: &SortedLetters) -> Option<SortedLetters> {
        self.sorted.minus(sorted)
    }

    pub fn rank(&self) -> Option<usize> {
        self.rank
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LetterMask(u32);

impl LetterMask {
    pub fn new(letters: &str) -> Self {
        let mut mask = 0;
        for ch in letters.chars() {
            if ch.is_alphabetic() {
                mask |= 1 << (ch.to_ascii_lowercase() as usize - 'a' as usize);
            }
        }
        Self(mask)
    }
}

impl TryFrom<&str> for LetterMask {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            Err(())
        } else {
            Ok(Self::new(s))
        }
    }
}

impl TryFrom<Option<&str>> for LetterMask {
    type Error = ();
    fn try_from(s: Option<&str>) -> Result<Self, Self::Error> {
        if let Some(s) = s {
            Self::try_from(s)
        } else {
            Err(())
        }
    }
}






pub fn search<'a>(lexi: &'a Lexicon, filter: &Filter) -> Vec<&'a str> {
    let mut entries: Vec<&'a Entry<'a>> = lexi
        .entries()
        .filter(|entry| filter.matches(entry))
        .collect();
    entries.sort_unstable_by(|a, b| {
        if a.len() == b.len() {
            a.word().cmp(b.word())
        } else {
            b.len().cmp(&a.len())
        }
    });
    let words: Vec<&'a str> = entries.iter().map(|entry| entry.word()).collect();
    words
}

pub fn anagram_breakdowns<'a>(
    lexi: &'a Lexicon<'a>,
    filter: &Filter,
    contains: &'a str,
) -> Vec<AnagramHit<'a>> {
    let sorted = SortedLetters::from_word(contains);
    lexi.entries()
        .filter(|entry| filter.matches(entry))
        .map(|entry| {
            let remainder = entry.without_letters_in(&sorted).unwrap();
            AnagramHit {
                query: contains,
                short: solve_anagram(lexi, &remainder),
                long: solve_anagram(lexi, &entry.sorted),
            }
        })
        .collect()
}

pub fn solve_anagram(lexi: &Lexicon, letters: &SortedLetters) -> RankedWord {
    let result = lexi
        .solve_anagram(letters)
        .min_by_key(|entry| match entry.rank() {
            Some(rank) => rank,
            None => lexi.len(),
        });

    match result {
        Some(entry) => {
            let word = entry.word().to_owned();
            let quality = match entry.rank() {
                Some(rank) if rank < 1500 => Quality::VeryPopular,
                Some(_) => Quality::LessPopular,
                None => Quality::NotPopular,
            };
            RankedWord { word, quality }
        }
        None => RankedWord {
            word: letters.to_string(),
            quality: Quality::NotWord,
        },
    }
}

pub fn search_countdown<'a>(lexi: &'a Lexicon<'a>, letters: &str) -> Vec<&'a str> {
    let sorted = SortedLetters::from_word(letters);
    let filter = FilterBuilder::new().contained(sorted).build();

    let mut entries: Vec<&'a Entry<'a>> = lexi
        .entries()
        .filter(|entry| filter.matches(entry))
        .filter(|entry| entry.word.chars().all(|c| c.is_ascii_lowercase()))
        .collect();

    // Sort by decreasing length, then by decreasing popularity, then alphabetically.
    entries.sort_unstable_by(|a, b| {
        if a.len() == b.len() {
            if a.rank() == b.rank() {
                a.word().cmp(b.word())
            } else {
                b.rank().cmp(&a.rank())
            }
        } else {
            b.len().cmp(&a.len())
        }
    });

    entries.into_iter().map(|entry| entry.word()).collect()
}

/// Represents part of an anagram search result.
/// For example, if the search query is "abc", and the result is "abcdef",
/// then the reduction is "abc" -> "def".
///
/// The remainder is the part of the result that is not in the search query.
pub struct AnagramHit<'a> {
    pub query: &'a str,
    // Short word that can be added to the search query to get the full result.
    pub short: RankedWord,
    // Long word containing the search term and remaining letters
    pub long: RankedWord,
}

pub struct RankedWord {
    pub word: String,
    pub quality: Quality,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Quality {
    NotWord = 0,
    NotPopular = 1,
    LessPopular = 2,
    VeryPopular = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Popularity {
    Low,
    Medium,
    High,
}

#[cfg(test)]
mod belle_tests {
    use super::*;
    use crate::FilterBuilder;
    use assert2::check;




    #[test]
    fn test_anagram_breakdown() {
        let filter = FilterBuilder::new().contains("lusters").length("8").build();
        let lexi = Lexicon::from_iter(["clusters", "dusters", "lusters"])
            .with_popular_words(["clusters"], 1);
        let xs = anagram_breakdowns(&lexi, &filter, "lusters");
        check!(xs.len() == 1);
        check!(xs[0].query == "lusters");
        check!(xs[0].short.word == "c");
        check!(xs[0].short.quality == Quality::NotWord);
        check!(xs[0].long.word == "clusters");
        check!(xs[0].long.quality == Quality::VeryPopular);
    }
}
