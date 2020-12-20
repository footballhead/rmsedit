use super::cga;
use super::ega;

/// Width and height
pub const IMAGE_DIMENSION: u32 = 15;
pub const IMAGE_DIMENSION_USIZE: usize = IMAGE_DIMENSION as usize;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Convenience function for compact construction (LOC-wise) with full opacity.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: 0xFF,
        }
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

pub fn load_spritesheet(filename: &str) -> Vec<Image> {
    fn header_match(to_match: &[u8]) -> [u8; 4] {
        [to_match[0], to_match[1], to_match[2], to_match[3]]
    }

    // TODO: optimize (e.g. don't load pic_data twice)
    let pic_data = std::fs::read(filename).unwrap();
    match header_match(&pic_data[0..4]) {
        cga::CGA_HEADER => return cga::load_spritesheet(filename),
        ega::EGA_HEADER => return ega::load_spritesheet(filename),
        _ => panic!("no matching header"), // TODO return result with error instead
    }
}
