use super::cga;
use super::ega;

/// Width and height
pub const IMAGE_DIMENSION: u32 = 15;

fn header_match(to_match: &[u8]) -> [u8; 4] {
    [to_match[0], to_match[1], to_match[2], to_match[3]]
}

pub fn load_spritesheet<'a>(
    filename: &str,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Vec<sdl2::render::Texture<'a>>
{
    // TODO: optimize
    let pic_data = std::fs::read(filename).unwrap();
    match header_match(&pic_data[0..4]) {
        cga::CGA_HEADER => return cga::load_spritesheet(filename, texture_creator),
        ega::EGA_HEADER => return ega::load_spritesheet(filename, texture_creator),
        _ => panic!("no matching header") // TODO return result with error instead
    }
}
