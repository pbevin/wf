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

#[derive(Debug, Clone, Serialize)]
pub struct PreviewResults {
    pub num_total: usize,
    pub num_shown: usize,
    pub groups: Vec<WordGroup>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(into = "PreviewGroupRepr")]
pub struct WordGroup {
    pub len: usize,
    pub words: Vec<(String, Popularity)>,
}

type PreviewGroupRepr = (usize, Vec<(String, Popularity)>);
impl From<WordGroup> for PreviewGroupRepr {
    fn from(group: WordGroup) -> Self {
        (group.len, group.words)
    }
}

struct WordSummary<'a> {
    word: &'a str,
    len: usize,
    rating: Popularity,
}

impl<'a> WordSummary<'a> {
    fn new(lexi: &Lexicon<'a>, entry: &Entry<'a>) -> Self {
        let word = entry.word();
        let len = word.len();
        let rating = lexi.rate(entry);
        Self { word, len, rating }
    }
}

/// Performs a search for the given query, and returns a short preview of the results.
/// This is intended to be used for live previews in the UI.
pub async fn get_preview(
    Query(query): Query<PreviewQuery>,
    Extension(lexi): Extension<Arc<Lexicon<'_>>>,
) -> Json<PreviewResults> {
    do_search(&query.term, 10, lexi)
}

pub async fn search(
    Query(query): Query<PreviewQuery>,
    Extension(lexi): Extension<Arc<Lexicon<'_>>>,
) -> Json<PreviewResults> {
    do_search(&query.term, usize::MAX, lexi)
}

fn do_search(term: &str, limit: usize, lexi: Arc<Lexicon>) -> Json<PreviewResults> {
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
    Json(PreviewResults {
        num_total,
        num_shown,
        groups,
    })
}

fn build_group<'a, T>((word_length, entries): (usize, T)) -> WordGroup
where
    T: IntoIterator<Item = WordSummary<'a>>,
{
    WordGroup {
        len: word_length,
        words: entries
            .into_iter()
            .map(|summary| (summary.word.to_owned(), summary.rating))
            .collect_vec(),
    }
}
