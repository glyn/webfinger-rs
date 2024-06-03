# webfinger-rs

See the following instructions for how to build and use the module.

## Building

This module is written in Rust. After [installing Rust](https://www.rust-lang.org/tools/install),
the module may be built using `cargo`.

For example, issue the following command in the root directory of a clone of this repository:
~~~
cargo build --release
~~~

This will build a shared library in `target/release`.

## Using

The `webfinger-rs` server is started by executing the following command:
~~~
webfinger-rs --jrd-map-path /path/to/jrdmap.json
~~~

where the file whose path is specified using `--jrd-map-path` is a JSON file containing a JSON map from string URI to the [JSON Resource Descriptor](https://www.rfc-editor.org/rfc/rfc7033.html#page-11) (JRD) associated with the URI.

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

## Contributing

See the [Contributor Guide](./CONTRIBUTING.md) if you'd like to submit changes.
