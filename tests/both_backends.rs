//! Isolated integration test. Skipped unless both backends are enabled:
//! `cargo test --features aws_lc_rs,rust_crypto --test both_backends`.

#![cfg(all(feature = "aws_lc_rs", feature = "rust_crypto"))]

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

#[test]
fn both_features_select_aws_lc_rs_without_install_default() {
    let claims = Claims { sub: "b@b.com".to_string(), exp: 9_999_999_999 };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret"))
        .expect("both backends should default to aws_lc_rs");
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(b"secret"),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .expect("round-trip with the automatic default");
    assert_eq!(decoded.claims, claims);
}
