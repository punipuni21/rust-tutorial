#[test]
pub fn assert_sample() {
    assert!(true);

    assert!(false, "panic! value={}", false);

    assert_eq!(true, true);
    assert_ne!(true, false);

    assert_eq!(true, false, "panic! value=({} {})", true, false);
}
