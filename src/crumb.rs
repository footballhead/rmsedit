pub const CRUMB_BITS: u8 = 2;
pub const CRUMB_MASK: u8 = 0x3;
pub const CRUMBS_PER_BYTE: usize = 4;

/// Extract a 2 bit subregion from an 8 bit value
///
/// # Arguments
///
/// * `val`: The 8-bit value from which to extract
/// * `part`: Which crumb subdivision of val to extract
///
/// # Examples
///
/// ```
/// let foo = 0b11_10_01_00u8;
/// assert_eq!(crumb(&foo, 0), 0b00u8);
/// assert_eq!(crumb(&foo, 1), 0b01u8);
/// assert_eq!(crumb(&foo, 2), 0b10u8);
/// assert_eq!(crumb(&foo, 3), 0b11u8);
/// ```
pub fn crumb(val: &u8, part: u8) -> u8 {
    // TODO: assert that part in [0..4)
    (val >> (CRUMB_BITS * part)) & CRUMB_MASK
}
