
mod aead_tests;
mod agreement_tests;
mod constant_time_tests;
mod digest_tests;

// ecdsa_tests not included because some file is in other directory
// test_file!("../src/ec/suite_b/ecdsa/ecdsa_sign_fixed_tests.txt"),
// test_file!("../src/ec/suite_b/ecdsa/ecdsa_sign_asn1_tests.txt"),
// mod ecdsa_tests;
mod ed25519_tests;
mod error_tests;
mod hkdf_tests;
mod hmac_tests;
mod pbkdf2_tests;
mod quic_tests;
mod rand_tests;

// Include Rsa tests will cause build failed.
// and also it required some file out for tests directory
// include_bytes!("../src/rsa/signature_rsa_example_private_key.der");
// Currently disable RSA tests.
// mod rsa_tests;
mod signature_tests;

#[test]
#[should_panic]
fn test_panic() {
    assert!(false)
}

#[test]
fn test_no_panic() {
    assert!(true)
}
