mod search;

use self::search::get_preview;
use self::search::search;
use crate::assets::static_path;
use crate::lexi::search_countdown;
use crate::lexi::Lexicon;
use crate::ServerOpts;
use axum::extract::Json;
use axum::extract::Query;
use axum::Extension;
use axum::{routing::get, Router};
use serde::Deserialize;
use serde::Serialize;
use std::process;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

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
    let lexi = Arc::new(Lexicon::load());

    let app = Router::new()
        .route("/api/preview", get(get_preview))
        .route("/api/results", get(search))
        .route("/api/countdown", get(countdown))
        .fallback(get(static_path))
        .layer(Extension(Arc::clone(&lexi)))
        .layer(TraceLayer::new_for_http());

    println!("Listening on {}", addr);

    // run it with hyper on the given address
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
    pub words: Vec<String>,
}

pub async fn countdown(
    Query(query): Query<CountdownQuery>,
    Extension(lexi): Extension<Arc<Lexicon<'_>>>,
) -> Json<CountdownResults> {
    let q = query.q;
    let words = search_countdown(&lexi, &q)
        .into_iter()
        .take(10)
        .map(|s| s.to_owned())
        .collect();
    Json(CountdownResults { q, words })
}
