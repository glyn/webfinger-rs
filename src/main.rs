/*
Copyright 2024 alice Normington

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
    body::Body, extract::State, http::StatusCode, response::Response, routing::get, Router,
};
use axum_extra::extract::Query;
use http::Uri;
use hyper::header::CONTENT_TYPE;
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
    resource: Vec<String>,

    #[serde(default)]
    rel: Vec<String>,
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

async fn handler(State(state): State<ServerState>, Query(params): Query<Params>) -> Response {
    let uri = params.resource;

    // "resource" parameter must be specified exactly once
    if uri.len() != 1 {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(""))
            .unwrap()
    } else {
        let uri = uri.get(0).unwrap();
        let parsed_uri = uri.parse::<Uri>();
        if parsed_uri.is_err() || parsed_uri.unwrap().scheme().is_none() {
            //panic!("{:?}", uri.parse::<Uri>().unwrap().scheme().unwrap());
            //panic!("{:?}", uri.parse::<Uri>().unwrap().scheme());
            // Malformed "resource" parameter
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(""))
                .unwrap()
        } else if let Some(jrd) = state.webfinger_jrdmap.get(uri) {
            let body = if params.rel.is_empty() {
                jrdmap::to_json(&jrd)
            } else {
                jrdmap::to_json(&jrd.filter(params.rel))
            };

            Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "application/jrd+json")
                .body(Body::from(body))
                .unwrap()
        } else {
            // URI not found
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(""))
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::str;
    use tokio::net::TcpListener;
    use tower::ServiceExt;

    #[tokio::test]
    async fn router_test() {
        let jm = jrdmap::from_json(
            &r#"
            {
                "acct:alice@example.com":{
                    "subject": "acct:alice@example.com",
                    "links": [
                        {
                            "rel": "http://webfinger.net/rel/avatar",
                            "type": "image/jpeg",
                            "href": "https://example.com/data/alice-avatar.jpeg"
                        }
                    ]
                }
            }"#
            .to_string(),
        );
        let router = create_router(jm);

        let response = router
            .oneshot(Request::builder().uri("/.well-known/webfinger?resource=acct:alice@example.com&rel=http://webfinger.net/rel/avatar").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "application/jrd+json"
        );

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual: Value = serde_json::from_str(str::from_utf8(&body[..]).unwrap()).unwrap();
        let expected = json!(
        {
            "subject":"acct:alice@example.com",
            "links": [
                {
                    "rel":"http://webfinger.net/rel/avatar",
                    "type":"image/jpeg",
                    "href":"https://example.com/data/alice-avatar.jpeg"
                }
            ]
        });
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn router_test_with_multiple_rels_in_query() {
        let jm = jrdmap::from_json(
            &r#"
            {
                "acct:alice@example.com":{
                    "subject": "acct:alice@example.com",
                    "links": [
                        {
                            "rel": "http://webfinger.net/rel/avatar",
                            "type": "image/jpeg",
                            "href": "https://example.com/data/alice-avatar.jpeg"
                        },
                        {
                            "rel": "me",
                            "href": "acct:me@example.com"
                        },
                        {
                            "rel": "author",
                            "href": "acct:author@example.com"
                        }
                    ]
                }
            }"#
            .to_string(),
        );
        let router = create_router(jm);

        let response = router
            .oneshot(Request::builder().uri("/.well-known/webfinger?resource=acct:alice@example.com&rel=http://webfinger.net/rel/avatar&rel=me").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "application/jrd+json"
        );

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual: Value = serde_json::from_str(str::from_utf8(&body[..]).unwrap()).unwrap();
        let expected = json!(
        {
            "subject":"acct:alice@example.com",
            "links": [
                {
                    "rel":"http://webfinger.net/rel/avatar",
                    "type":"image/jpeg",
                    "href":"https://example.com/data/alice-avatar.jpeg"
                },
                {
                    "rel": "me",
                    "href": "acct:me@example.com"
                }
            ]
        });
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn router_test_with_encoded_query() {
        let jm = jrdmap::from_json(
            &r#"
            {
                "acct:alice@example.com":{
                    "subject": "acct:alice@example.com",
                    "links": [
                        {
                            "rel": "http://webfinger.net/rel/avatar",
                            "type": "image/jpeg",
                            "href": "https://example.com/data/alice-avatar.jpeg"
                        }
                    ]
                }
            }"#
            .to_string(),
        );
        let router = create_router(jm);

        let response = router
            .oneshot(Request::builder().uri("/.well-known/webfinger?resource=acct%3Aalice%40example.com&rel=http%3a//webfinger.net/rel/avatar").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "application/jrd+json"
        );

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual: Value = serde_json::from_str(str::from_utf8(&body[..]).unwrap()).unwrap();
        let expected = json!(
        {
            "subject":"acct:alice@example.com",
            "links": [
                {
                    "rel":"http://webfinger.net/rel/avatar",
                    "type":"image/jpeg",
                    "href":"https://example.com/data/alice-avatar.jpeg"
                }
            ]
        });
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn not_found() {
        let jm = jrdmap::from_json(
            &r#"
            {
                "acct:other@example.com":{
                    "subject": "acct:other@example.com"
                }
            }"#
            .to_string(),
        );
        let router = create_router(jm);

        let response = router
            .oneshot(
                Request::builder()
                    .uri("/.well-known/webfinger?resource=acct:alice@example.com")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn missing_resource() {
        let jm = jrdmap::from_json(
            &r#"
            {
                "acct:other@example.com":{
                    "subject": "acct:other@example.com"
                }
            }"#
            .to_string(),
        );
        let router = create_router(jm);

        let response = router
            .oneshot(
                Request::builder()
                    .uri("/.well-known/webfinger")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn duplicate_resource() {
        let jm = jrdmap::from_json(
            &r#"
            {
                "acct:other@example.com":{
                    "subject": "acct:other@example.com"
                }
            }"#
            .to_string(),
        );
        let router = create_router(jm);

        let response = router
            .oneshot(
                Request::builder()
                    .uri("/.well-known/webfinger?resource=acct:other@example.com&resource=acct:other@example.com")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn malformed_resource() {
        let jm = jrdmap::from_json(
            &r#"
            {
                "acct:other@example.com":{
                    "subject": "acct:other@example.com"
                }
            }"#
            .to_string(),
        );
        let router = create_router(jm);

        let response = router
            .oneshot(
                Request::builder()
                    .uri("/.well-known/webfinger?resource=other@example.com") // resource not a URI // FIXME: this appears to be a URI??
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn invalid_rel() {
        let jm = jrdmap::from_json(
            &r#"
            {
                "acct:other@example.com":{
                    "subject": "acct:other@example.com"
                }
            }"#
            .to_string(),
        );
        let router = create_router(jm);

        let response = router
            .oneshot(
                Request::builder()
                    .uri("/.well-known/webfinger?resource=acct:alice@example.com&rel=")
                    .body(Body::empty())
                    .unwrap(),
            )
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
                    "acct:alice@example.com":{
                        "subject": "acct:alice@example.com",
                        "links": [
                            {
                                "rel":"http://webfinger.net/rel/avatar",
                                "type":"image/jpeg",
                                "href":"https://example.com/data/alice-avatar.jpeg"
                            }
                        ]
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
                        "http://{addr}/.well-known/webfinger?resource=acct://alice@example.com"
                    ))
                    .header("Host", "localhost")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "application/jrd+json"
        );

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual: Value = serde_json::from_str(str::from_utf8(&body[..]).unwrap()).unwrap();
        let expected = json!({"subject":"acct:alice@example.com",
            "links": [
                {
                    "rel":"http://webfinger.net/rel/avatar",
                    "type":"image/jpeg",
                    "href":"https://example.com/data/alice-avatar.jpeg"
                }
            ]
        });
        assert_eq!(actual, expected);
    }
}
