use crate::lexi::anagram_breakdowns;
use itertools::Itertools;

use crate::lexi::AnagramHit;
use crate::lexi::Filter;
use crate::lexi::FilterBuilder;
use crate::lexi::Lexicon;
use crate::lexi::Quality;
use axum::extract::Json;
use axum::extract::Query;
use axum::Extension;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

const MAX_RESULTS: usize = 100;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum SearchResults {
    /// A simple list of words, sorted by decreasing length, and then alphabetically.
    #[serde(rename = "words")]
    Words { words: Vec<String> },
    /// A list of possible anagram breakdowns.
    #[serde(rename = "anagrams")]
    Anagram { term: String, hits: Vec<Hit> },
    /// The server refuses to return that many results.
    #[serde(rename = "oversize")]
    Oversize { max_hits: usize },
}

#[derive(Debug, Clone, Serialize)]
pub struct Hit {
    query: String,
    score: usize,
    short: (String, usize),
    long: (String, usize),
}

impl<'a> From<AnagramHit<'a>> for Hit {
    fn from(AnagramHit { query, short, long }: AnagramHit<'a>) -> Self {
        let query = query.into();
        let score = score(short.quality, long.quality);
        let long = (long.word.to_string(), long.quality as usize);
        let short = (short.word.to_string(), short.quality as usize);
        Hit {
            query,
            long,
            short,
            score,
        }
    }
}

fn score(short: Quality, long: Quality) -> usize {
    let s1 = long as usize;
    let s2 = short as usize * 4;
    s1 * s1 + s2 * s2
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchQuery {
    contains: Option<String>,
    contained: Option<String>,
    excluded: Option<String>,
    included: Option<String>,
    length: Option<String>,
    #[serde(rename = "isOneWord")]
    oneword: Option<bool>,
}

impl SearchQuery {
    pub fn get_contains(&self) -> Option<&str> {
        if let Some(ref contains) = self.contains {
            if contains.len() >= 3 {
                return Some(contains);
            }
        }
        None
    }

    pub fn to_filter(&self) -> Filter {
        FilterBuilder::new()
            .contains(self.contains.as_deref())
            .contained(self.contained.as_deref())
            .exclude_letters(self.excluded.as_deref())
            .include_letters(self.included.as_deref())
            .length(self.length.as_deref())
            .single_word(self.oneword)
            .build()
    }
}

pub async fn fullsearch(
    Query(query): Query<SearchQuery>,
    Extension(lexi): Extension<Arc<Lexicon<'static>>>,
) -> Json<SearchResults> {
    let filter = query.to_filter();

    if let Some(term) = query.get_contains() {
        let hits = anagram_breakdowns(&lexi, &filter, term);
        if hits.len() > MAX_RESULTS {
            return Json(SearchResults::Oversize {
                max_hits: MAX_RESULTS,
            });
        }
        let mut hits = hits.into_iter().map(Hit::from).collect_vec();
        // Sort hits by decreasing score
        hits.sort_by(|a, b| b.score.cmp(&a.score));

        Json(SearchResults::Anagram {
            term: term.into(),
            hits,
        })
    } else {
        let words = crate::lexi::search(&lexi, &filter)
            .into_iter()
            .map(|w| w.to_owned())
            .collect();
        Json(SearchResults::Words { words })
    }
}
