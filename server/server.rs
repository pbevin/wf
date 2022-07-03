use crate::assets::static_path;
use crate::lexi::anagram_breakdowns;
use crate::lexi::search_countdown;
use crate::lexi::AnagramHit;
use crate::lexi::Filter;
use crate::lexi::FilterBuilder;
use crate::lexi::Lexicon;
use crate::lexi::Quality;
use crate::ServerOpts;
use axum::extract::Json;
use axum::extract::Query;
use axum::{routing::get, Router};
use itertools::Itertools;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use std::process;
use tower_http::trace::TraceLayer;

const MAX_RESULTS: usize = 100;

// #[tokio::main]
pub fn start_sync(opts: &ServerOpts) {
    ctrlc::set_handler(move || {
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(start(opts));
}

pub async fn start(opts: &ServerOpts) {
    let addr = match &opts.addr {
        Some(addr) => addr.to_owned(),
        None => "127.0.0.1:3000".to_owned(),
    };
    let addr = addr.parse::<std::net::SocketAddr>().unwrap();

    let app = Router::new()
        .route("/api/search", get(search))
        .route("/api/countdown", get(countdown))
        .fallback(get(static_path))
        .layer(TraceLayer::new_for_http());

    println!("Listening on {}", addr);

    // run it with hyper on the given address
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn search(Query(query): Query<SearchQuery>) -> Json<SearchResults> {
    let filter = query.to_filter();

    if let Some(term) = query.get_contains() {
        let hits = anagram_breakdowns(&LEXI, &filter, term);
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
        let words = crate::lexi::search(&LEXI, &filter);
        Json(SearchResults::Words { words })
    }
}

/// Countdown query
#[derive(Deserialize)]
pub struct CountdownQuery {
    pub q: String,
}

/// Countdown results
#[derive(Serialize)]
pub struct CountdownResults {
    pub q: String,
    pub words: Vec<&'static str>,
}

pub async fn countdown(Query(query): Query<CountdownQuery>) -> Json<CountdownResults> {
    let q = query.q;
    let words = search_countdown(&LEXI, &q).into_iter().take(10).collect();
    Json(CountdownResults { q, words })
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum SearchResults {
    /// A simple list of words, sorted by decreasing length, and then alphabetically.
    #[serde(rename = "words")]
    Words { words: Vec<&'static str> },
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

lazy_static! {
    static ref LEXI: Lexicon<'static> = Lexicon::load();
}

// lazy_static! {
//     static ref SEARCH_SERVICE: SearchService<'static> =
//         SearchService::new(WORDS, POPULAR_WORDS, 1500);
// }
