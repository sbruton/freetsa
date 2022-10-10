use simple_asn1::{ASN1Block, BigUint, OID};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimestampApiError {
    #[error("http client failure: {}", _0)]
    HttpClient(#[source] reqwest::Error),
    #[error("api rejected request: {}", _0)]
    Remote(#[source] reqwest::Error),
    #[error("failed to encore timestamp request: {}", _0)]
    RequestEncoding(#[from] simple_asn1::ASN1EncodeErr),
    #[error("failure receiving response: {}", _0)]
    Response(#[source] reqwest::Error),
}

#[cfg(feature = "file")]
#[derive(Debug, Error)]
pub enum TimestampFileError {
    #[error("failed to read file: {}", _0)]
    FileIo(#[source] std::io::Error),
    #[error("{}", _0)]
    Api(#[from] TimestampApiError),
}

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

pub struct TimestampResponse {
    pub query: Vec<u8>,
    pub reply: Vec<u8>,
}

pub mod prelude {
    pub use super::timestamp_hash;
    pub use super::TimestampResponse;

    #[cfg(feature = "file")]
    pub use super::timestamp_file;
}
