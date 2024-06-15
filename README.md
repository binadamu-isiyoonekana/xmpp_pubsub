# Humble Xmpp PubSub Protocol Parser

## How to build

For building a release version of the package, enter the following command:

```sh
$cargo build --release
```

The final executable, called `xmpp`, is located in `target/release`.

## How to run

For running the parser (e.g. during development) using Cargo, proceed as follows:

```sh
$ cargo run
```

Once built (see [How to build](#how-to-build) paragraph earlier in this document), the `xmpp` application can be executed as follows:

```sh
$ cargo build --release
$ ./target/release/xmpp
```

## Sample Xmpp pubsub payloads

Some Xmpp pubsub payloads are defined in [constants.rs](./src/constants.rs) file. See the [main.rs](./src/bin/main.rs) to get some insights on how to use the [XmppPubSubParser](./src/parser.rs).

## Encoding support

Encoding will be inferred from the XML declaration if it is found, otherwise UTF-8 is assumed.

Currently, only ASCII-compatible encodings are supported. For instance, the UTF-16 format is not supported because the underlying `quick-xml` reader is not [standard compliant]. Consequently, this Xmpp parser supports all encodings provided by the [`encoding_rs`] crate, except these:

- [UTF-16BE]
- [UTF-16LE]
- [ISO-2022-JP]

You should stop processing a document when one of these encodings is detected, because generated events can be wrong and do not reflect a real document structure!

This restriction will be eliminated once issue [#158] is resolved.

[standard compliant]: https://www.w3.org/TR/xml11/#charencoding
[UTF-16BE]: encoding_rs::UTF_16BE
[UTF-16LE]: encoding_rs::UTF_16LE
[ISO-2022-JP]: encoding_rs::ISO_2022_JP
[#158]: https://github.com/tafia/quick-xml/issues/158

Please refer to the `quick-xml` [features](https://github.com/tafia/quick-xml/blob/master/README.md#features) for more information.
