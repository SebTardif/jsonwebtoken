//! Isolated integration test. Skipped unless both backends are enabled:
//! `cargo test --features aws_lc_rs,rust_crypto --test both_backends`.

#![cfg(all(feature = "aws_lc_rs", feature = "rust_crypto"))]

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

#[derive(Serialize)]
struct Claims {
    sub: &'static str,
}

#[test]
fn both_features_without_install_default_returns_error() {
    let err = encode(
        &Header::default(),
        &Claims { sub: "b@b.com" },
        &EncodingKey::from_secret(b"secret"),
    )
    .expect_err("both backends without install_default must not pick one");
    assert_eq!(*err.kind(), ErrorKind::MissingCryptoProvider);
}
