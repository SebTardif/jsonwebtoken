//! Isolated integration test. Skipped unless neither backend feature is on:
//! `cargo test --no-default-features --test missing_provider`.
//! The rest of `tests/` needs a backend feature.

#![cfg(not(any(feature = "aws_lc_rs", feature = "rust_crypto")))]

use jsonwebtoken::crypto::{CryptoProvider, KeyUtils};
use jsonwebtoken::errors::{ErrorKind, new_error};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

#[derive(Serialize)]
struct Claims {
    sub: &'static str,
}

static TEST_PROVIDER: CryptoProvider = CryptoProvider {
    signer_factory: |_, _| Err(new_error(ErrorKind::Provider("installed-test".to_string()))),
    verifier_factory: |_, _| Err(new_error(ErrorKind::Provider("installed-test".to_string()))),
    key_utils: KeyUtils::new_unimplemented(),
};

#[test]
fn encode_without_provider_returns_error_and_leaves_install_slot() {
    let err = encode(
        &Header::default(),
        &Claims { sub: "b@b.com" },
        &EncodingKey::from_secret(b"secret"),
    )
    .expect_err("missing CryptoProvider must not panic");
    assert_eq!(*err.kind(), ErrorKind::MissingCryptoProvider);

    CryptoProvider::install_default(&TEST_PROVIDER)
        .expect("a failed encode must not consume the install slot");

    let err = encode(
        &Header::default(),
        &Claims { sub: "b@b.com" },
        &EncodingKey::from_secret(b"secret"),
    )
    .expect_err("test provider is installed but does not sign");
    match err.kind() {
        ErrorKind::Provider(msg) => {
            assert!(msg.contains("installed-test"), "unexpected Provider message: {msg}");
        }
        other => panic!("expected Provider after install_default, got {other:?}"),
    }
}
