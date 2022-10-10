//! # FreeTSA Client Library

use simple_asn1::{ASN1Block, BigUint, OID};
use thiserror::Error;

/// Errors that can be generated while interacting with the FreeTSA API
#[derive(Debug, Error)]
pub enum TimestampApiError {
    /// HTTP client failed before API request could be made
    #[error("http client failure: {}", _0)]
    HttpClient(#[source] reqwest::Error),
    /// FreeTSA rejected the timestamp request
    #[error("api rejected request: {}", _0)]
    Remote(#[source] reqwest::Error),
    /// Failed to ASN.1/DER encode the timestamp request
    #[error("failed to encore timestamp request: {}", _0)]
    RequestEncoding(#[from] simple_asn1::ASN1EncodeErr),
    /// Failed to process the FreeTSA API response
    #[error("failure receiving response: {}", _0)]
    Response(#[source] reqwest::Error),
}

/// Errors that can be generated while timestamping a file
///
/// *This type is available only if freetsa is built with the `"file"` feature.*
#[cfg(feature = "file")]
#[derive(Debug, Error)]
pub enum TimestampFileError {
    /// I/O failure reading the file to be timestamped
    #[error("failed to read file: {}", _0)]
    FileIo(#[source] std::io::Error),
    /// FreeTSA API failure
    #[error("{}", _0)]
    Api(#[from] TimestampApiError),
}

/// Timestamp a file.
///
/// This method generates a SHA512 hash of the specified file and submits it
/// to FreeTSA to be timestamped.
///
/// *This method is available only if freetsa is built with the `"file"` feature.*
///
/// __Example__
/// ```rust,no_run
/// use freetsa::prelude::*;
/// use tokio::fs::OpenOptions;
/// use tokio::io::AsyncWriteExt;
///
/// #[tokio::main]
/// async fn main() {
///     // request timestamp with automatically generated file hash
///     let TimestampResponse { reply, .. } = timestamp_file("path/to/file").await.unwrap();
///     // create file where we'll persist the timestamp reply
///     let mut reply_file = OpenOptions::new()
///         .create(true)
///         .write(true)
///         .open("example.tsr")
///         .await
///         .unwrap();
///     // write timestamp reply to file
///     reply_file.write_all(&reply).await.unwrap();
///     // ensure os has completed writing all data
///     reply_file.flush().await.unwrap();
/// }
/// ```
#[cfg(feature = "file")]
pub async fn timestamp_file(
    path: impl AsRef<std::path::Path>,
) -> Result<TimestampResponse, TimestampFileError> {
    use sha2::{Digest, Sha512};
    let file = tokio::fs::read(path)
        .await
        .map_err(TimestampFileError::FileIo)?;
    let mut hasher = Sha512::new();
    hasher.update(file);
    let hash = hasher.finalize();

    Ok(timestamp_hash(hash.to_vec()).await?)
}

/// Timestamp a hash
///
/// This method takes a SHA512 hash and submits it to FreeTSA to be timestamped.
///
/// __Example__
/// ```rust,no_run
/// use freetsa::prelude::*;
/// use futures_util::TryFutureExt;
/// use tokio::try_join;
/// use tokio::fs::OpenOptions;
/// use tokio::io::AsyncWriteExt;
///
/// #[tokio::main]
/// async fn main() {
///     // generate a hash in some manner, here we use a literal as an example
///     let hash = hex_literal::hex!("401b09eab3c013d4ca54922bb802bec8fd5318192b0a75f201d8b3727429080fb337591abd3e44453b954555b7a0812e1081c39b740293f765eae731f5a65ed1").to_vec();
///     // request timestamp with pre-generated hash
///     let TimestampResponse { reply, query } = timestamp_hash(hash).await.unwrap();
///     // create file where we'll persist the timestamp query
///     let mut query_file = OpenOptions::new();
///     let query_file = query_file.create(true).write(true).open("example.tsq");
///     // create file where we'll persist the timestamp reply
///     let mut reply_file = OpenOptions::new();
///     let reply_file = reply_file.create(true).write(true).open("example.tsr");
///     // wait on all data writes
///     try_join!(
///         async move {
///             let mut query_file = query_file.await?;
///             query_file.write_all(&query).await?;
///             query_file.flush().await
///         },
///         async move {
///             let mut reply_file = reply_file.await?;
///             reply_file.write_all(&reply).await?;
///             reply_file.flush().await
///         }
///     ).unwrap();
/// }
/// ```
pub async fn timestamp_hash(hash: Vec<u8>) -> Result<TimestampResponse, TimestampApiError> {
    let sha512_oid: Vec<BigUint> = [2u16, 16, 840, 1, 101, 3, 4, 2, 3]
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();
    let req = ASN1Block::Sequence(
        3,
        vec![
            ASN1Block::Integer(1, 1.into()),
            ASN1Block::Sequence(
                2,
                vec![
                    ASN1Block::Sequence(
                        2,
                        vec![
                            ASN1Block::ObjectIdentifier(1, OID::new(sha512_oid)),
                            ASN1Block::Null(1),
                        ],
                    ),
                    ASN1Block::OctetString(1, hash),
                ],
            ),
            ASN1Block::Boolean(1, true),
        ],
    );
    let req = simple_asn1::to_der(&req)?;
    let client = reqwest::ClientBuilder::new()
        .build()
        .map_err(TimestampApiError::HttpClient)?;
    let response = client
        .post("https://freetsa.org/tsr")
        .header("content-type", "application/timestamp-query")
        .body(req.clone())
        .send()
        .await
        .map_err(TimestampApiError::Remote)?;
    let payload = response
        .bytes()
        .await
        .map_err(TimestampApiError::Response)?;
    Ok(TimestampResponse {
        query: req,
        reply: payload.into(),
    })
}

/// Timestamp API response
pub struct TimestampResponse {
    /// Timestamp query, ASN.1/DER encoded, as sent to FreeTSA API
    pub query: Vec<u8>,
    /// Timestamp response, ASN.1/DER encoded, as received from FreeTSA API
    pub reply: Vec<u8>,
}

pub mod prelude {
    pub use super::timestamp_hash;
    pub use super::TimestampResponse;

    #[cfg(feature = "file")]
    pub use super::timestamp_file;
}
