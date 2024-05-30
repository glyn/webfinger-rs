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

use std::convert::Infallible;
use std::fs;
use std::net::SocketAddr;

use clap::Parser;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path of webfinger JSON file
    #[arg(short, long)]
    file_path: String,
}

type Hellofn = fn(Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible>;

fn make_hello(jrd : String) -> Hellofn {
    || {
        Ok(Response::new(Full::new(Bytes::from(jrd))))
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

fn main() {
    let args = Args::parse();

    let webfinger_jrd = fs::read_to_string(args.file_path)
        .expect("Failed to read file");

    let rt = tokio::runtime::Runtime::new().unwrap();

    let result : Result<(), Box<dyn std::error::Error + Send + Sync>>  = rt.block_on(async {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        
        // We create a TcpListener and bind it to 127.0.0.1:3000
        let listener = TcpListener::bind(addr).await?;
        
        // We start a loop to continuously accept incoming connections
        loop {
            let (stream, _) = listener.accept().await?;
            
            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);
            
            // Spawn a tokio task to serve multiple connections concurrently
            tokio::task::spawn(async move {
                // Finally, we bind the incoming connection to our `hello` service
                if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(make_hello(webfinger_jrd)))
                .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    });
    result.unwrap();
}
