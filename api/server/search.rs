use crate::lexi::Entry;
use crate::lexi::FilterBuilder;
use crate::lexi::Lexicon;
use crate::lexi::Popularity;
use axum::extract::Query;
use axum::Extension;
use axum::Json;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameType {
    Countdown,
    Connect,
    Anagram,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewQuery {
    #[serde(rename = "q")]
    pub term: String,
    pub goal: GameType,
    pub limit: Option<usize>,
}

/// The result of a preview query or full search.
#[derive(Debug, Clone, Serialize)]
pub struct SearchResults {
    pub num_total: usize,
    pub num_shown: usize,
    #[serde(flatten)]
    pub results: Results,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Results {
    WordsByLength { groups: Vec<WordGroup> },
    // Anagrams(Vec<Decomposition>),
}

/// A group of words with the same length.
#[derive(Debug, Clone, Serialize)]
pub struct WordGroup {
    /// All the words in the group are this length.
    pub len: usize,
    /// Words in the group, most popular first.
    pub words: Vec<RatedWord>,
}

/// A collection of ways to decompose a word into subsets,
/// such that all the subsets except possibly the last are
/// in the lexicon.
/// For example, some of the decompositions of STEAM are:
///   ME + AT + "S"
///   ME + SAT + ""
///   SEA + "TM"
///   A + STEM + ""
/// The quoted component at the end is the "remainder" that
/// is not in the lexicon.
#[derive(Debug, Clone, Serialize)]
pub struct Decomposition {
    pub words: Vec<RatedWord>,
    pub remainder: String,
}

/// A word, plus a rough estimate of its popularity.  The word is guaranteed
/// to be in the lexicon, and the rating is a rough estimate of how popular
/// the word is (1 = rare, 2 = common, 3 = very common)
#[derive(Debug, Clone, Serialize)]
pub struct RatedWord {
    pub word: String,
    pub rating: Popularity,
}

/// Performs a search for the given query, and returns a short preview of the results.
/// This is intended to be used for live previews in the UI.
pub async fn get_preview(
    Query(query): Query<PreviewQuery>,
    Extension(lexi): Extension<Arc<Lexicon<'_>>>,
) -> Json<SearchResults> {
    do_search(&query.term, 10, lexi)
}

pub async fn search(
    Query(query): Query<PreviewQuery>,
    Extension(lexi): Extension<Arc<Lexicon<'_>>>,
) -> Json<SearchResults> {
    do_search(&query.term, usize::MAX, lexi)
}

fn do_search(term: &str, limit: usize, lexi: Arc<Lexicon>) -> Json<SearchResults> {
    let filter = FilterBuilder::new()
        .contained(term)
        .single_word(true.into())
        .build();
    let mut results = lexi
        .entries()
        .filter(|entry| filter.matches(entry))
        .map(|entry| WordSummary::new(&lexi, entry))
        .collect_vec();
    let num_total = results.len();
    results.sort_unstable_by_key(|s| Reverse((s.len, s.rating)));
    let shown = results.into_iter().take(limit).collect_vec();
    let num_shown = shown.len();
    let groups = shown
        .into_iter()
        .group_by(|s| s.len)
        .into_iter()
        .map(build_group)
        .collect_vec();
    Json(SearchResults {
        num_total,
        num_shown,
        results: Results::WordsByLength { groups },
    })
}

struct WordSummary<'a> {
    word: &'a str,
    len: usize,
    rating: Popularity,
}

impl<'a> WordSummary<'a> {
    pub fn new(lexi: &'a Lexicon, entry: &'a Entry) -> Self {
        let word = entry.word();
        let len = word.len();
        let rating = lexi.rate(entry);
        Self { word, len, rating }
    }
}

fn build_group<'a, T>((word_length, entries): (usize, T)) -> WordGroup
where
    T: IntoIterator<Item = WordSummary<'a>>,
{
    WordGroup {
        len: word_length,
        words: entries
            .into_iter()
            .map(|summary| RatedWord {
                word: summary.word.to_owned(),
                rating: summary.rating,
            })
            .collect_vec(),
    }
}
