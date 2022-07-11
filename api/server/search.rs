use super::types::Decomposition;
use super::types::{CountedResults, GameType, RatedWord, SearchQuery, SearchResults, WordGroup};
use crate::lexi::Entry;
use crate::lexi::FilterBuilder;
use crate::lexi::Lexicon;
use crate::lexi::Popularity;
use axum::extract::Query;
use axum::Extension;
use axum::Json;
use itertools::Itertools;
use std::cmp::Reverse;
use std::sync::Arc;

pub async fn search(
    Query(query): Query<SearchQuery>,
    Extension(lexi): Extension<Arc<Lexicon<'_>>>,
) -> Json<CountedResults> {
    let limit = query.limit.unwrap_or(usize::MAX);
    match query.game_type {
        GameType::Countdown | GameType::Connect => longest_subwords(&query.term, limit, lexi),
        GameType::Anagram => anagram_search(&query.term, limit, lexi),
        GameType::Ghost => todo!(),
    }
}

fn longest_subwords(term: &str, limit: usize, lexi: Arc<Lexicon>) -> Json<CountedResults> {
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
    Json(CountedResults {
        num_total,
        num_shown,
        results: SearchResults::WordsByLength { groups },
    })
}

fn anagram_search(term: &str, limit: usize, lexi: Arc<Lexicon>) -> Json<CountedResults> {
    let results = crate::anagrams(term, &lexi)
        .filter_map(|(words, residue)| residue.is_empty().then(|| words))
        .collect_vec();
    let num_total = results.len();
    let mut results = results
        .iter()
        .map(|entries| Decomposition {
            words: entries
                .iter()
                .map(|entry| {
                    let word = entry.word().into();
                    let rating = lexi.rate(entry);
                    RatedWord { word, rating }
                })
                .collect_vec(),
        })
        .collect_vec();

    // Stable sort here, because there was already a vague ordering by
    // quality of the first word.
    results.sort_by_key(|r| {
        let most_unpopular = r
            .words
            .iter()
            .map(|w| w.rating as usize)
            .min()
            .unwrap_or_default();
        // let pop1 = r.words.get(0).map(|w| w.rating as usize).unwrap_or(0);
        // let popularity = r.words.iter().map(|w| w.rating as usize).max().unwrap_or(0);
        // let unpopularity = r.words.iter().map(|w| w.rating as usize).min().unwrap_or(5);
        // let max_len = r.words.iter().map(|w| w.word.len()).max().unwrap_or(0);
        // let min_len = r.words.iter().map(|w| w.word.len()).min().unwrap_or(20);
        (Reverse(most_unpopular), r.words.len())
    });

    results.truncate(limit);

    Json(CountedResults {
        num_total,
        num_shown: results.len(),
        results: SearchResults::Anagrams { anagrams: results },
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
