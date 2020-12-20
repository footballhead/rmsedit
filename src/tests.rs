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

#[test]
fn pascal_from_pascal_string_test() {
    use super::pascal::from_pascal_string;

    assert_eq!(from_pascal_string(b"\x0BHello world"), "Hello world");
    assert_eq!(from_pascal_string(b"\x0AHello world"), "Hello worl");
}

#[test]
fn pascal_to_pascal_string_test() {
    use super::pascal::to_pascal_string;

    assert_eq!(to_pascal_string("", 0xFF), b"\x00");
    assert_eq!(to_pascal_string("Hello world", 0x00), b"\x00");
    assert_eq!(to_pascal_string("Hello world", 0xFF), b"\x0BHello world");
}
