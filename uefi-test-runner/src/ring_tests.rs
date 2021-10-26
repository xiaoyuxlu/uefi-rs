
mod digest_tests;

#[test]
#[should_panic]
fn test_panic() {
    assert!(false)
}

#[test]
fn test_no_panic() {
    assert!(true)
}
