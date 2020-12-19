#[test]
fn crumb_basic_test() {
    use super::crumb::crumb;

    let foo = 0b00_10_11_01u8;
    assert_eq!(crumb(&foo, 0), 0b01u8);
    assert_eq!(crumb(&foo, 1), 0b11u8);
    assert_eq!(crumb(&foo, 2), 0b10u8);
    assert_eq!(crumb(&foo, 3), 0b00u8);
    // TODO: test crumb(&foo, 4) panics
}
