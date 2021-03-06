use super::crumb;
use super::crumb::crumb;
use super::img;

pub const CGA_HEADER: [u8; 4] = [0x0E, 0x00, 0x0E, 0x00];

const IMAGE_ROW_CRUMBS: usize = 16;
const IMAGE_ROW_SIZE: usize = IMAGE_ROW_CRUMBS / crumb::CRUMBS_PER_BYTE;

const CGA_IMAGE_SIZE: usize = 64;
const IMAGE_ALIGNMENT: usize = CGA_IMAGE_SIZE * 4;
// TODO: static_assert(CGA_IMAGE_SIZE < IMAGE_ALIGNMENT)

// TODO: static_assert(IMAGE_DIMENSION <= IMAGE_ROW_NIBBLES)
// TODO: static_assert((CGA_IMAGE_SIZE - IMAGE_ROW_SIZE) / IMAGE_ROW_SIZE == IMAGE_DIMENSION)

// TODO: prefer "dark" CGA palette (whatever DOSBox uses)
const CGA_PALETTE: [img::Color; 4] = [
    img::Color::rgb(0x00, 0x00, 0x00),
    img::Color::rgb(0x00, 0xFF, 0xFF),
    img::Color::rgb(0xFF, 0x00, 0xFF),
    img::Color::rgb(0xFF, 0xFF, 0xFF),
];

pub fn load_spritesheet(filename: &str) -> Vec<img::Image> {
    let pic_data = std::fs::read(filename).unwrap();
    return pic_data
        // Divide the stream of bytes into discrete image sections.
        .chunks(IMAGE_ALIGNMENT)
        // TODO: Validate and throw away chunks that don't match expected size
        // Ignore first row (CGA_HEADER, 4 bytes) and garbage after image data.
        // (IDK why CGA data is only 1/4 of the allotted space... ask John Murphy)
        .map(|x| &x[IMAGE_ROW_SIZE..CGA_IMAGE_SIZE])
        // TODO: Validate and throw away chunks that don't match expected size
        // Turn byte chunks into images
        .map(|x| {
            img::Image {
                width: img::IMAGE_DIMENSION_USIZE,
                height: img::IMAGE_DIMENSION_USIZE,
                pixels: x
                    .iter()
                    // Turn 1 byte into 4 crumbs
                    .flat_map(|xx| vec![crumb(xx, 3), crumb(xx, 2), crumb(xx, 1), crumb(xx, 0)])
                    // The last crumb of each row is garbage
                    .enumerate()
                    .filter(|&(i, _)| i % IMAGE_ROW_CRUMBS < (img::IMAGE_DIMENSION as usize))
                    // Draw pixels
                    .map(|(_, x)| CGA_PALETTE[x as usize])
                    .collect(),
            }
        })
        .collect();
}
