//! Isolated integration test. Skipped unless neither backend feature is on:
//! `cargo test --no-default-features --test missing_provider`.
//! The rest of `tests/` needs a backend feature.

#![cfg(not(any(feature = "aws_lc_rs", feature = "rust_crypto")))]

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

#[derive(Serialize)]
struct Claims {
    sub: &'static str,
}

#[test]
fn encode_without_provider_returns_error() {
    let err = encode(
        &Header::default(),
        &Claims { sub: "b@b.com" },
        &EncodingKey::from_secret(b"secret"),
    )
    .expect_err("missing CryptoProvider must not panic");
    assert_eq!(*err.kind(), ErrorKind::MissingCryptoProvider);
}
