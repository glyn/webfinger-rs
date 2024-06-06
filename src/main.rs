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

mod jrdmap;
mod rel;

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
    /// File path of webfinger JRD map file
    #[arg(short, long)]
    jrd_map_path: String,

    /// Port number to listen on
    #[arg(short, long)]
    port: u16,
}

#[derive(Clone)]
struct ServerState {
    webfinger_jrdmap : jrdmap::JrdMap,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let webfinger_jrdmap = fs::read_to_string(args.jrd_map_path)
    .expect("Failed to read file");

    let jm = jrdmap::from_json(&webfinger_jrdmap);

    let state = ServerState { webfinger_jrdmap : jm};

    let router = Router::new()
        .route("/", get(handler))
        .with_state(state);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await?;
    println!("Listening on http://{}", listener.local_addr()?);

    axum::serve(listener, router).await
}

async fn handler(
    State(state): State<ServerState>,
) -> String {
    // use `state`...

    let uri = "acct:glyn@underlap.org".to_string();

    let jrd = state.webfinger_jrdmap.get(&uri).expect("No JRD found for input URI");

    let rel = "http://webfinger.net/rel/avatar".to_string();

    jrdmap::to_json(&jrd.filter(rel))
}