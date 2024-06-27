<img width=130 src="https://upload.wikimedia.org/wikipedia/commons/thumb/e/e6/WebFinger_Logo.svg/440px-WebFinger_Logo.svg.png">

# webfinger-rs

`webfinger-rs` is a simple [WebFinger](https://www.rfc-editor.org/rfc/rfc7033.html) server, written in Rust.

The server can support multiple WebFinger resources, but is intended for use with a small, relatively static number of such resources, for example on a personal website. The server is not intended for sites with many users, since the mappings from WebFinger resources to JSON Resource Descriptor (JRD) are stored in a single file.

WebFinger must be served over HTTPS, but this server currently supports only HTTP, so **this server must sit behind a HTTPS server**. For example, this server could be used in conjunction with a reverse proxy, such as NGINX or freenginx, that terminates HTTPS traffic from clients and then passes requests to this server.

## Requests and responses

Requests must contain the path component `/.well-known/webfinger`. Any other path component will result in HTTP 404 (Not Found).

Requests must contain a query component with exactly one `resource` parameter set to the value of the URI of the WebFinger resource being queried. If the `resource` parameter is absent, malformed, or if there is more than one `resource` parameter, this will result in HTTP 400 (Bad Request). If the `resource` parameter does not correspond to a known WebFinger resource, this will result in HTTP 404 (Not Found).

The query component of a request may contain one or more `rel` parameters. These are used to request a subset of the JRD for the WebFinger resource: only the links in the JRD with relation types matching the value of one of the `rel` parameters are returned. If no `rel` parameter is provided, all links in the JRD are returned.

Each parameter value in the request is percent-encoded as described in section [4.1](https://www.rfc-editor.org/rfc/rfc7033.html#section-4.1) of RFC 7033.

Any parameters of the query component other than `resource` and `rel` are ignored.

A successful response is indicated by HTTP 200 (OK) and includes the HTTP headers `Access-Control-Allow-Origin: *` and `Content-Type: application/jrd+json`. The response body consists of the JRD, or a subset of the JRD if the request included `rel` parameters. 

## Building

This server is written in Rust. After [installing Rust](https://www.rust-lang.org/tools/install),
build the server by issuing the `cargo` command in the root directory of a clone of this repository:
~~~
cargo build --release
~~~

This will build the `webfinger-rs` server executable in `target/release`.

## Usage

Start the `webfinger-rs` server by executing the following command:
~~~
webfinger-rs --port <portnum> --jrd-map-path /path/to/jrdmap.json
~~~

where `<portnum>` is the port the server should listen on and `--jrd-map-path` is the file path of a JSON file containing a map from string URI to the [JSON Resource Descriptor](https://www.rfc-editor.org/rfc/rfc7033.html#page-11) (JRD) associated with the URI.

For example:
~~~
{ "acct:alice@example.com": {
    "subject": "acct:alice@example.com",
    "aliases": [
      "acct:someone@example.com"
    ],
    "links": [
      {
        "rel": "http://webfinger.net/rel/avatar",
        "type": "image/jpeg",
        "href": "https://example.com/alice-avatar.jpeg"
      }
    ]
  },
  "acct:bob@example.com": {
    "subject": "acct:bob@example.com",
    "aliases": [
      "acct:somebody@example.com"
    ],
    "links": [
      {
        "rel": "self",
        "type": "application/activity+json",
        "href": "https://example.com/users/bob"
      }
    ]
  }
}
~~~

In the example, each URI in the top-level map is an account equal to the subject, but the URIs need not be accounts and need not be equal to the subject. See the WebFinger [RFC 7033](https://www.rfc-editor.org/rfc/rfc7033.html) for more information about URIs and subjects and [RFC 7565](https://www.rfc-editor.org/rfc/rfc7565.html) for details of the 'acct' URI scheme.

## Contributing

See the [Contributor Guide](./CONTRIBUTING.md) if you'd like to submit changes.
