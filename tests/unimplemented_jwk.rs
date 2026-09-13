use jsonwebtoken::crypto::KeyUtils;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::jwk::ThumbprintHash;

#[test]
fn unimplemented_jwk_utils_report_provider_not_missing() {
    let utils = KeyUtils::new_unimplemented();
    let err = (utils.compute_digest)(&[], ThumbprintHash::SHA256).unwrap_err();
    match err.kind() {
        ErrorKind::Provider(msg) => {
            assert!(msg.contains("JWKs"), "unexpected Provider message: {msg}");
        }
        other => panic!("expected Provider, got {other:?}"),
    }
}
