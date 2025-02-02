use protestr::provide;

#[provide]
fn test_provide() {
    assert_eq!(1, 1);
}
