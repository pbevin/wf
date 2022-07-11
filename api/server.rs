mod search;
mod types;

use self::search::search;
use crate::assets::static_path;
use crate::lexi::Lexicon;
use crate::ServerOpts;
use axum::Extension;
use axum::{routing::get, Router};
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
        .route("/api/search", get(search))
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
