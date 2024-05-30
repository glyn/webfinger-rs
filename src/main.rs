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

use clap::Parser;
use warp::Filter;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path of webfinger JSON file
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let webfinger_jrd = fs::read_to_string(args.file_path)
        .expect("Failed to read file");

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        // GET /hello/warp => 200 OK with body "Hello, warp!"
        let hello = warp::path!("hello" / String)
            .map(|name| format!("Hello, {} with {}!", name, webfinger_jrd));

        warp::serve(hello)
            .run(([127, 0, 0, 1], 3030))
            .await;
    });
}
