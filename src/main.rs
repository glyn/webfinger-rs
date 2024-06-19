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
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::fs;
use std::io;
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
    webfinger_jrdmap: jrdmap::JrdMap,
}

#[derive(Deserialize)]
struct Params {
    #[serde(default)]
    resource: String,
    rel: Option<String>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let webfinger_jrdmap = fs::read_to_string(args.jrd_map_path).expect("Failed to read file");

    let jm = jrdmap::from_json(&webfinger_jrdmap);

    let router = create_router(jm);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await?;
    println!("Listening on http://{}", listener.local_addr()?);

    axum::serve(listener, router).await
}

fn create_router(jm: jrdmap::JrdMap) -> Router {
    let state = ServerState {
        webfinger_jrdmap: jm,
    };

    Router::new()
        .route("/.well-known/webfinger", get(handler))
        .with_state(state)
}

async fn handler(State(state): State<ServerState>, Query(params): Query<Params>) -> String {
    // TODO: decode resource query value
    let uri = params.resource;

    let jrd = state
        .webfinger_jrdmap
        .get(&uri)
        .expect("No JRD found for input URI");

    if let Some(rel) = params.rel {
        jrdmap::to_json(&jrd.filter(rel))
    } else {
        jrdmap::to_json(&jrd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use pretty_assertions::{assert_eq, assert_ne};
    use serde_json::{json, Value};
    use std::str;
    use tokio::net::TcpListener;
    use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn router_test() {
        let jm = jrdmap::from_json(
            &r#"
            {
                "acct:glyn@underlap.org":{
                    "subject": "acct:glyn@underlap.org",
                    "links": [
                        {
                            "rel": "http://webfinger.net/rel/avatar",
                            "type": "image/jpeg",
                            "href": "https://underlap.org/data/glyn-avatar.jpeg"
                        }
                    ]
                }
            }"#
            .to_string(),
        );
        let router = create_router(jm);

        let response = router
            //.oneshot(Request::builder().uri("/.well-known/webfinger?resource=acct%3Aglyn%40underlap.org").body(Body::empty()).unwrap())
            .oneshot(Request::builder().uri("/.well-known/webfinger?resource=acct:glyn@underlap.org&rel=http://webfinger.net/rel/avatar").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual = str::from_utf8(&body[..]).unwrap();
        let expected = r#"{"subject":"acct:glyn@underlap.org","links":[{"rel":"http://webfinger.net/rel/avatar","type":"image/jpeg","href":"https://underlap.org/data/glyn-avatar.jpeg"}]}"#;
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn not_found() {
        let jm = jrdmap::from_json(
            &r#"
            {
                "acct:other@underlap.org":{
                    "subject": "acct:other@underlap.org"
                }
            }"#
            .to_string(),
        );
        let router = create_router(jm);

        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn integration_test() {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let jm = jrdmap::from_json(
                &r#"
                {
                    "acct:glyn@underlap.org":{
                        "subject": "acct:glyn@underlap.org"
                    }
                }"#
                .to_string(),
            );
            axum::serve(listener, create_router(jm)).await.unwrap();
        });

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();

        let response = client
            .request(
                Request::builder()
                    .uri(format!(
                        "http://{addr}/.well-known/webfinger?resource=acct:glyn@underlap.org"
                    ))
                    .header("Host", "localhost")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual = str::from_utf8(&body[..]).unwrap();
        assert_eq!(actual, r#"{"subject":"acct:glyn@underlap.org"}"#);
    }
}
