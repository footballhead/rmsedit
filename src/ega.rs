use super::crumb;
use super::crumb::crumb;
use super::img;

pub const EGA_HEADER: [u8; 4] = [0x1D, 0x00, 0x0E, 0x00];

const IMAGE_ALIGNMENT: usize = 256;
// TODO static_assert IMAGE_DIMENSION * IMAGE_DIMENSION + EGA_HEADER.len() <= IMAGE_ALIGNMENT

const IMAGE_CHANNEL_ROW_CRUMBS: usize = 16;
// TODO: static_assert IMAGE_DIMENSION <= IMAGE_CHANNEL_ROW_CRUMBS
const IMAGE_CHANNELS: usize = 4;

const IMAGE_ROW_CRUMBS: usize = IMAGE_CHANNEL_ROW_CRUMBS * IMAGE_CHANNELS;
const IMAGE_ROW_BYTES: usize = IMAGE_ROW_CRUMBS / crumb::CRUMBS_PER_BYTE;
const IMAGE_BYTES: usize = IMAGE_ROW_BYTES * img::IMAGE_DIMENSION_USIZE;
// TODO assert EGA_IMAGE_BYTES + EGA_HEADER.len() <= IMAGE_ALIGNMENT

const EGA_PALETTE: [img::Color; 16] = [
    img::Color::rgb(0x00, 0x00, 0x00),
    img::Color::rgb(0x00, 0x00, 0xAA),
    img::Color::rgb(0x00, 0xAA, 0x00),
    img::Color::rgb(0x00, 0xAA, 0xAA),
    img::Color::rgb(0xAA, 0x00, 0x00),
    img::Color::rgb(0xAA, 0x00, 0xAA),
    img::Color::rgb(0xAA, 0x55, 0x00),
    img::Color::rgb(0xAA, 0xAA, 0xAA),
    img::Color::rgb(0x55, 0x55, 0x55),
    img::Color::rgb(0x55, 0x55, 0xFF),
    img::Color::rgb(0x55, 0xFF, 0x55),
    img::Color::rgb(0x55, 0xFF, 0xFF),
    img::Color::rgb(0xFF, 0x55, 0x55),
    img::Color::rgb(0xFF, 0x55, 0xFF),
    img::Color::rgb(0xFF, 0xFF, 0x55),
    img::Color::rgb(0xFF, 0xFF, 0xFF),
];

// Lifetime: the returned Textures have data owned by TextureCreator
pub fn load_spritesheet(filename: &str) -> Vec<img::Image> {
    let pic_data = std::fs::read(filename).unwrap();
    return pic_data
        // Divide the stream of bytes into discrete image sections.
        .chunks(IMAGE_ALIGNMENT)
        // Ignore EGA_HEADER and garbage between images
        .map(|x| &x[EGA_HEADER.len()..IMAGE_BYTES + EGA_HEADER.len()])
        // Turn byte chunks into images
        .map(|x| {
            // Use an intermediate EGA buffer to accumulate bit channels
            // TODO: Is there a more Rust-idiomatic way of doing this?
            // I want a stride iterator.
            // I can imagine a zip of row chunks but use of chunks() is limited...
            let mut ega_color_buffer =
                [0u8; img::IMAGE_DIMENSION_USIZE * img::IMAGE_DIMENSION_USIZE];
            x.iter()
                // Turn 1 byte into 4 crumbs
                .flat_map(|xx| vec![crumb(xx, 3), crumb(xx, 2), crumb(xx, 1), crumb(xx, 0)])
                // Collapse 2 bit value into 1 bit value.
                // A crumb is either 0b00 or 0b11
                // Please, John Murphy, why
                .map(|xx| if xx > 0 { 1 } else { 0 })
                // The last crumb of each channel row is garbage
                .enumerate()
                .filter(|&(i, _)| i % IMAGE_CHANNEL_ROW_CRUMBS < img::IMAGE_DIMENSION_USIZE)
                // Accumulate pixels
                .for_each(|(i, v)| {
                    // Since i is leftover from the previous filter, we use that row size
                    // (IMAGE_ROW_CRUMBS) instead of the actual row size (IMAGE_DIMENSION)
                    let xx = i % IMAGE_CHANNEL_ROW_CRUMBS;
                    let yy = i / IMAGE_ROW_CRUMBS;

                    // Figure out which channel we are in to do the appropriate bit shifts
                    let shift_amount =
                        IMAGE_CHANNELS - 1 - ((i % IMAGE_ROW_CRUMBS) / IMAGE_CHANNEL_ROW_CRUMBS);
                    ega_color_buffer[yy * img::IMAGE_DIMENSION_USIZE + xx] |= v << shift_amount;
                });

            img::Image {
                width: img::IMAGE_DIMENSION_USIZE,
                height: img::IMAGE_DIMENSION_USIZE,
                pixels: ega_color_buffer
                    .iter()
                    .map(|x| EGA_PALETTE[*x as usize])
                    .collect(),
            }
        })
        .collect();
}
