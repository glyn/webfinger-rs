# webfinger-rs

`webfinger-rs` is a simple [WebFinger](https://www.rfc-editor.org/rfc/rfc7033.html) server for personal websites, written in Rust.

The server can support multiple URIs, but the intention is there is a small, relatively static number of such URIs. The server is not intended for sites with many users since the mappings from URIs to JSON Resource Descriptor are stored in a single file.

See the following instructions for how to build and use the server.

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

In the example, each URI in the top-level map is an account equal to the subject, but the two values are not necessarily equal. 

## Contributing

See the [Contributor Guide](./CONTRIBUTING.md) if you'd like to submit changes.
