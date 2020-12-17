use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
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

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Hello", 640, 480)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut textures: Vec<sdl2::render::Texture> = Vec::new();

    // TODO: Write this in a more functional way
    let pic = std::fs::read("CGAPICS.PIC").unwrap();
    for image_chunk in pic.chunks(IMAGE_ALIGNMENT) {
        // TODO: Is there a "best" pixel format for what I'm doing?
        let mut surface =
            Surface::new(IMAGE_DIMENSION, IMAGE_DIMENSION, PixelFormatEnum::RGB24).unwrap();

        // Ignore first row (4 byte header + 4 bytes garbage) and garbage after image data
        // IDK Why the CGA image data is only a fraction of the allotted space... ask John Murphy
        let chunk_only_image_data = &image_chunk[IMAGE_ROW_SIZE..CGA_IMAGE_SIZE];

        for (y, row_data) in chunk_only_image_data.chunks(IMAGE_ROW_SIZE).enumerate() {
            for (x, pixel) in row_data.iter().enumerate() {
                // Turn byte reads into crumbs
                let a = (pixel >> CRUMB_BITS >> CRUMB_BITS >> CRUMB_BITS) & CRUMB_MASK;
                let b = (pixel >> CRUMB_BITS >> CRUMB_BITS) & CRUMB_MASK;
                let c = (pixel >> CRUMB_BITS) & CRUMB_MASK;
                let d = (pixel) & CRUMB_MASK;

                surface
                    .fill_rect(
                        sdl2::rect::Rect::new((x * CRUMBS_PER_BYTE) as i32, y as i32, 1, 1),
                        CGA_PALETTE[a as usize],
                    )
                    .unwrap();
                surface
                    .fill_rect(
                        sdl2::rect::Rect::new((x * CRUMBS_PER_BYTE + 1) as i32, y as i32, 1, 1),
                        CGA_PALETTE[b as usize],
                    )
                    .unwrap();
                surface
                    .fill_rect(
                        sdl2::rect::Rect::new((x * CRUMBS_PER_BYTE + 2) as i32, y as i32, 1, 1),
                        CGA_PALETTE[c as usize],
                    )
                    .unwrap();
                // TODO: less fragile way of handling incomplete rows
                if (x * CRUMBS_PER_BYTE + 3) < (IMAGE_DIMENSION as usize) {
                    surface
                        .fill_rect(
                            sdl2::rect::Rect::new((x * CRUMBS_PER_BYTE + 3) as i32, y as i32, 1, 1),
                            CGA_PALETTE[d as usize],
                        )
                        .unwrap();
                }
            }
        }

        textures.push(surface.as_texture(&texture_creator).unwrap());
    }

    canvas.clear();
    canvas.copy(&textures[0], None, None).unwrap();
    canvas.present();

    let mut debug_image_index = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'mainloop: loop {
        for event in event_pump.wait_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'mainloop,
                _ => {
                    canvas.clear();
                    canvas.copy(&textures[debug_image_index], None, None).unwrap();
                    canvas.present();
                }
            }

            debug_image_index = (debug_image_index + 1) % textures.len();
        }
    }
}
