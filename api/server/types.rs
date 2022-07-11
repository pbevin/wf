use crate::lexi::Popularity;
use serde::{Deserialize, Serialize};

/// A word game that we can suggest moves for.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameType {
    /// Find the longest word
    Countdown,
    /// Find the longest word
    Connect,
    /// Find anagrams of the given word
    Anagram,
    /// Find words that contain the given word plus one new letter
    Ghost,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchQuery {
    #[serde(rename = "q")]
    pub term: String,
    #[serde(rename = "goal")]
    pub game_type: GameType,
    pub limit: Option<usize>,
}

/// The result of a preview query or full search.
#[derive(Debug, Clone, Serialize)]
pub struct CountedResults {
    pub num_total: usize,
    pub num_shown: usize,
    #[serde(flatten)]
    pub results: SearchResults,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SearchResults {
    WordsByLength { groups: Vec<WordGroup> },
    Anagrams { anagrams: Vec<Decomposition> },
}

/// A group of words with the same length.
#[derive(Debug, Clone, Serialize)]
pub struct WordGroup {
    /// All the words in the group are this length.
    pub len: usize,
    /// Words in the group, most popular first.
    pub words: Vec<RatedWord>,
}

/// A collection of ways to decompose a word into smaller words,
/// with all of the words in the decomposition being in the lexicon.
///
/// For example, some of the decompositions of STEAM are:
///   MATES
///   ME + SAT
///   SET + AM
#[derive(Debug, Clone, Serialize)]
pub struct Decomposition {
    pub words: Vec<RatedWord>,
}

/// A word, plus a rough estimate of its popularity.  The word is guaranteed
/// to be in the lexicon, and the rating is a rough estimate of how popular
/// the word is (1 = rare, 2 = common, 3 = very common)
#[derive(Debug, Clone, Serialize)]
pub struct RatedWord {
    pub word: String,
    pub rating: Popularity,
}
