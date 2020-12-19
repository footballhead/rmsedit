use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

const CRUMB_BITS: u8 = 2;
const CRUMB_MASK: u8 = 0x3;
const CRUMBS_PER_BYTE: usize = 4;

const IMAGE_ROW_CRUMBS: usize = 16;
const IMAGE_ROW_SIZE: usize = IMAGE_ROW_CRUMBS / CRUMBS_PER_BYTE;

// const CGA_HEADER: [u8; 4] = [0x0E, 0x00, 0x0E, 0x00];
const CGA_IMAGE_SIZE: usize = 64;
const IMAGE_ALIGNMENT: usize = CGA_IMAGE_SIZE * 4;
// TODO: static_assert(CGA_IMAGE_SIZE < IMAGE_ALIGNMENT)

// Width and height
const IMAGE_DIMENSION: u32 = 15;
// TODO: static_assert(IMAGE_DIMENSION <= IMAGE_ROW_NIBBLES)
// TODO: static_assert((CGA_IMAGE_SIZE - IMAGE_ROW_SIZE) / IMAGE_ROW_SIZE == IMAGE_DIMENSION)

// TODO: prefer "dark" CGA palette (whatever DOSBox uses)
const CGA_PALETTE: [Color; 4] = [
    Color::RGB(0x00, 0x00, 0x00),
    Color::RGB(0x00, 0xFF, 0xFF),
    Color::RGB(0xFF, 0x00, 0xFF),
    Color::RGB(0xFF, 0xFF, 0xFF),
];

// higher part = more significant bit
// e.g. 33221100
fn crumb(val: &u8, part: u8) -> u8 {
    (val >> (CRUMB_BITS * part)) & CRUMB_MASK
}

fn pixel(x: i32, y: i32) -> Rect {
    Rect::new(x, y, 1, 1)
}

// Lifetime: the returned Textures have data owned by TextureCreator
pub fn load_spritesheet<'a>(
    filename: &str,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Vec<sdl2::render::Texture<'a>> {
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
            // TODO: Is there a "best" pixel format for what I'm doing?
            let mut surface =
                Surface::new(IMAGE_DIMENSION, IMAGE_DIMENSION, PixelFormatEnum::RGB24).unwrap();

            x.iter()
                // Turn 1 byte into 4 crumbs
                .flat_map(|xx| vec![crumb(xx, 3), crumb(xx, 2), crumb(xx, 1), crumb(xx, 0)])
                // The last crumb of each row is garbage
                .enumerate()
                .filter(|&(i, _)| i % IMAGE_ROW_CRUMBS < (IMAGE_DIMENSION as usize))
                // Draw pixels
                .for_each(|(i, x)| {
                    surface
                        .fill_rect(
                            // Since i is leftover from the previous filter, we use that row size
                            // (IMAGE_ROW_CRUMBS) instead of the actual row size (IMAGE_DIMENSION)
                            pixel((i % IMAGE_ROW_CRUMBS) as i32, (i / IMAGE_ROW_CRUMBS) as i32),
                            CGA_PALETTE[x as usize],
                        )
                        .unwrap()
                });

            // Return a texture
            // This is the only place texture_creator is used
            // To eliminate dependencies, could return a Vec<Surface> then convert outside this fn
            surface.as_texture(&texture_creator).unwrap()
        })
        .collect();
}