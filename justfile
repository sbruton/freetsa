# fetch current trust anchor certs from https://freetsa.org
@_sync_certs:
    if [ ! -e tsa.crt ]; then \
        echo "WARN: fetching TSA certificate from https://freetsa.org/files/tsa.crt" 1>&2; \
        echo "-- USE THESE AT YOUR OWN RISK, VISIT https://freetsa.org TO VALIDATE CERTS" 1>&2; \
        echo 1>&2; \
        curl -q https://freetsa.org/files/tsa.crt > tsa.crt; \
    fi
    if [ ! -e cacert.pem ]; then \
        echo "WARN: fetching CA certificate from https://freetsa.org/files/cacert.pem" 1>&2; \
        echo "-- USE THESE AT YOUR OWN RISK, VISIT https://freetsa.org TO VALIDATE CERTS" 1>&2; \
        echo 1>&2; \
        curl -q https://freetsa.org/files/cacert.pem > cacert.pem; \
    fi

# delete all build and example artifacts
clean:
    cargo clean
    if [ -e example.tsr ]; then rm example.tsr; fi
    if [ -e example.tsq ]; then rm example.tsq; fi

# generate html documentation for the library
doc:
    cargo doc --no-deps --all-features --open

# timestamp Cargo.toml using the example code examples/file.rs
example-file: _sync_certs
    cargo run --example file --features file

# timestamp a hash using the example code examples/hash.rs
example-hash: _sync_certs
    cargo run --example hash