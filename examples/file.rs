use freetsa::prelude::*;
use std::{fs::OpenOptions, io::Write};

#[tokio::main]
async fn main() {
    // submit a timestamp request, if successful you'll be given the DER-encoded query and reply
    let TimestampResponse { query: _, reply } = timestamp_file("Cargo.toml").await.unwrap();

    // persist the reply however you need, here we save to disk
    let mut reply_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./example.tsr")
        .unwrap();
    reply_file.write_all(&reply).unwrap();
    reply_file.flush().unwrap();

    // try validating the timestamp response
    println!("Download cacert.pem and tsa.crt from https://freetsa.org, then");
    println!("run `openssl ts -verify -in example.tsr -data Cargo.toml -CAfile cacert.pem -untrusted tsa.crt`");
}
