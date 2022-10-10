use freetsa::prelude::*;
use hex_literal::hex;
use std::{fs::OpenOptions, io::Write};

#[tokio::main]
async fn main() {
    // load or generate the has to sign however is appropriate for your solution
    let my_hash: Vec<u8> = hex!("8d3fffddf79e9a232ffd19f9ccaa4d6b37a6a243dbe0f23137b108a043d9da13121a9b505c804956b22e93c7f93969f4a7ba8ddea45bf4aab0bebc8f814e0991").into();

    // submit a timestamp request, if successful you'll be given the DER-encoded query and reply
    let TimestampResponse { query, reply } = timestamp_hash(my_hash).await.unwrap();

    // persist the reply and query however you need, here we save to disk
    let mut query_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./example.tsq")
        .unwrap();
    let mut reply_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./example.tsr")
        .unwrap();
    query_file.write_all(&query).unwrap();
    reply_file.write_all(&reply).unwrap();
    query_file.flush().unwrap();
    reply_file.flush().unwrap();

    // try validating the timestamp response
    println!("Download cacert.pem and tsa.crt from https://freetsa.org, then");
    println!("run `openssl ts -verify -in example.tsr -queryfile example.tsq -CAfile cacert.pem -untrusted tsa.crt`");
}
