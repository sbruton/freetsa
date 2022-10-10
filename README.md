# FreeTSA Unofficial Client Library and CLI Utility

<a href="https://crates.io/crates/freetsa"><img alt="Crate Info" src="https://img.shields.io/crates/v/freetsa.svg"/></a>
<a href="https://docs.rs/freetsa/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-freetsa-green"/></a>
<a href="https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html"><img alt="Rustc Version 1.60+" src="https://img.shields.io/badge/rustc-1.60%2B-lightgrey.svg"/></a>

See [https://freetsa.org] for more information on this public timestamp service.

_Note: To verify timestamps, you will need to fetch copies of FreeTSA's certificates from their website._

## Using CLI

```shell
$ cargo install freetsa

$ freetsa timestamp file \
    --data some_file \
    --reply-out some_file.tsr \
    --query-out some_file.tsq

$ openssl ts -verify \
    -in some_file.tsr \
    -queryfile some_file.tsq \
    -CAfile cacert.pem \
    -untrusted tsa.crt
```

## Using Library

```rust
use freetsa::prelude::*;

// timestamp a hash that you generate
let hash: Vec<u8> = _generate_your_hash_somehow();
let TimestampResponse { reply, .. } = timestamp_hash(hash).await.unwrap();

// timestamp a sha512 hash generated for you from a file you specify
let TimestampResponse { query, reply } = timestamp_file("path/to/my/file").await.unwrap();
```

Example code is available for [timestamping a file] or [timestamping a hash]. You can run them using [just] with `just example-file` and `just example-hash`, respectively.

[https://freetsa.org]: https://freetsa.org
[timestamping a file]: examples/file.rs
[timestamping a hash]: examples/hash.rs
[just]: https://github.com/casey/just
