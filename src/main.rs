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
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, Router},
};
use std::io;
use tokio::net::TcpListener;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path of webfinger JSON file
    #[arg(short, long)]
    file_path: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    
    let router = Router::new()
        .route("/", get("Hello world!"));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on http://{}", listener.local_addr()?);

    axum::serve(listener, router).await
}
