/// Convert a byte length-prefixed string into a Rust string.
/// Only single byte lengths are supported (max 255 chars)
///
/// https://en.wikipedia.org/wiki/String_(computer_science)#Length-prefixed
pub fn from_pascal_string(pstring: &[u8]) -> String {
    let length = pstring[0] as usize;
    String::from_utf8(pstring[1..length + 1].to_vec()).unwrap()
}
