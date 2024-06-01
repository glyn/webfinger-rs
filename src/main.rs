/*
Copyright 2024 Glyn Normington

This file is part of webfinger-rs.

webfinger-rs is free software: you can redistribute it and/or modify it under the terms
of the GNU General Public License as published by the Free Software Foundation, either
version 3 of the License, or (at your option) any later version.

webfinger-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with webfinger-rs.
If not, see <https://www.gnu.org/licenses/>.
*/

use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get},
};
use std::io;
use std::fs;
use tokio::net::TcpListener;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path of webfinger JSON file
    #[arg(short, long)]
    file_path: String,
}

// the application state
//
// here you can put configuration, database connection pools, or whatever
// state you need
//
// see "When states need to implement `Clone`" for more details on why we need
// `#[derive(Clone)]` here.
#[derive(Clone)]
struct AppState {
    webfinger_jrd : String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let webfinger_jrd = fs::read_to_string(args.file_path)
    .expect("Failed to read file");

    let state = AppState { webfinger_jrd : webfinger_jrd};

    let router = Router::new()
        .route("/", get(handler))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on http://{}", listener.local_addr()?);

    axum::serve(listener, router).await
}

async fn handler(
    // access the state via the `State` extractor
    // extracting a state of the wrong type results in a compile error
    State(state): State<AppState>,
) -> String {
    // use `state`...

    state.webfinger_jrd.to_string()
}