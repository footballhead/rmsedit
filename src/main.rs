use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::surface::Surface;

mod cga;
mod crumb;
mod ega;
mod img;
mod monster;
mod rms;

mod tests;

struct PaintEvent {}

struct EditorView {
    /// X-coordinate on screen in pixels (origin is top-left)
    pub x: u32,
    /// Y-coordinate on screen in pixels (origin is top-left)
    pub y: u32,
    /// Width and height of on-screen tiles, in pixels
    pub tile_dimensions: u32,
}

impl EditorView {
    fn rect(&self, x: u32, y: u32) -> Rect {
        Rect::new(
            (self.x + x * self.tile_dimensions) as i32,
            (self.y + y * self.tile_dimensions) as i32,
            self.tile_dimensions,
            self.tile_dimensions,
        )
    }
    fn localize(&self, x: i32, y: i32) -> (i32, i32) {
        (
            (x - self.x as i32) / self.tile_dimensions as i32,
            (y - self.y as i32) / self.tile_dimensions as i32,
        )
    }
}

fn apply_mask(image: &mut img::Image, mask_image: &img::Image) {
    image
        .pixels
        .iter_mut()
        .zip(mask_image.pixels.iter())
        .for_each(|(pixel, mask)| {
            if *mask == img::Color::rgb(0xFF, 0xFF, 0xFF) {
                pixel.a = 0;
            }
        });
}

// TODO: Return Result<> since multiple operations can fail?
fn as_texture<'a, T>(image: &img::Image, texture_creator: &'a TextureCreator<T>) -> Texture<'a> {
    fn pixel(x: i32, y: i32) -> Rect {
        Rect::new(x, y, 1, 1)
    }
    fn wrap_color(color: &img::Color) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a)
    }
    // TODO: Is there a best pixel format for what I'm doing?
    // E.g. is ARGB8888 better than ABGR8888? What about ARGB32? How do I tell?
    let mut surface = Surface::new(
        image.width as u32,
        image.height as u32,
        PixelFormatEnum::ARGB8888,
    )
    .unwrap();

    image.pixels.iter().enumerate().for_each(|(i, x)| {
        surface
            .fill_rect(
                pixel((i % image.width) as i32, (i / image.width) as i32),
                wrap_color(x),
            )
            .unwrap()
    });

    return surface.as_texture(texture_creator).unwrap();
}

/// Convenience function for pushing a paint event
fn request_paint(event_subsystem: &sdl2::EventSubsystem) {
    event_subsystem.push_custom_event(PaintEvent {}).unwrap();
}

fn main() {
    let editor_view = EditorView {
        x: 8,
        y: 16,
        tile_dimensions: img::IMAGE_DIMENSION * 2,
    };
    let mut rooms = rms::load_rooms("DUNGEON.RMS");
    let monsters = monster::load_monsters("PYMON.DAT");

    let sdl_context = sdl2::init().unwrap();

    let event_subsystem = sdl_context.event().unwrap();
    event_subsystem
        .register_custom_event::<PaintEvent>()
        .unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Hello", 640, 480)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut monster_color = img::load_spritesheet("PYMON.PIC");
    let monster_mask = img::load_spritesheet("PYMASK.PIC");
    monster_color
        .iter_mut()
        .zip(monster_mask.iter())
        .for_each(|(color, mask)| apply_mask(color, mask));

    // EGAPICS/CGAPICS contains both color data and masks. In order to use apply_mask
    // (inside Rust's borrow rules) I load a second immutable copy. Alternatively, I could
    // just implement Copy on Image... Or I could return a copy from apply_mask and not take
    // a mutable reference
    let mut tiles_color = img::load_spritesheet("EGAPICS.PIC");
    let tiles_mask = img::load_spritesheet("EGAPICS.PIC");
    // Only a handful of EGAPICS/CGAPICS tiles have masks.
    vec![
        (10, 64), // Attack effect
        (11, 69), // Hit explosion
        (18, 71), // Old bones
        (22, 65), // Treasure chest
        (23, 70), // Old body
        (24, 68), // Player
        (47, 67), // Smoke
        (50, 72), // Old stone coffin
        (55, 66), // Old grave
        (60, 82), // ???
        (75, 73), // ???
        (76, 74), // ???
        (83, 84), // Some old blood
    ]
    .iter()
    .for_each(|(tile, mask)| apply_mask(&mut tiles_color[*tile - 1], &tiles_mask[*mask - 1]));

    let tiles_atlas: Vec<sdl2::render::Texture> = tiles_color
        .iter()
        .map(|x| as_texture(x, &texture_creator))
        .collect();
    let monsters_atlas: Vec<sdl2::render::Texture> = monster_color
        .iter()
        .map(|x| as_texture(x, &texture_creator))
        .collect();

    let mut room_index: usize = 0;
    let mut is_dragging = false;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'mainloop: loop {
        for event in event_pump.wait_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {
                    scancode: Some(Scancode::Up),
                    ..
                } => {
                    if rooms[room_index].nav_north > 0 {
                        room_index = (rooms[room_index].nav_north - 1) as usize;
                    }
                    request_paint(&event_subsystem)
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Down),
                    ..
                } => {
                    if rooms[room_index].nav_south > 0 {
                        room_index = (rooms[room_index].nav_south - 1) as usize;
                    }
                    request_paint(&event_subsystem)
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Right),
                    ..
                } => {
                    if rooms[room_index].nav_east > 0 {
                        room_index = (rooms[room_index].nav_east - 1) as usize;
                    }
                    request_paint(&event_subsystem)
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Left),
                    ..
                } => {
                    if rooms[room_index].nav_west > 0 {
                        room_index = (rooms[room_index].nav_west - 1) as usize;
                    }
                    request_paint(&event_subsystem)
                }
                Event::KeyDown {
                    scancode: Some(Scancode::C),
                    ..
                } => {
                    if rooms[room_index].nav_up > 0 {
                        room_index = (rooms[room_index].nav_up - 1) as usize;
                    } else if rooms[room_index].nav_down > 0 {
                        room_index = (rooms[room_index].nav_down - 1) as usize;
                    }
                    request_paint(&event_subsystem)
                }
                Event::MouseButtonDown {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    is_dragging = true;
                    let (room_x, room_y) = editor_view.localize(x, y);
                    if room_x >= 0
                        && room_y >= 0
                        && room_x < rms::ROOM_WIDTH as i32
                        && room_y < rms::ROOM_HEIGHT as i32
                    {
                        rooms[room_index].set_tile(room_x as u32, room_y as u32, 1);
                        request_paint(&event_subsystem);
                    }
                }
                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    is_dragging = false;
                }
                Event::MouseMotion { x, y, .. } => {
                    if !is_dragging {
                        break;
                    }
                    let (room_x, room_y) = editor_view.localize(x, y);
                    if room_x >= 0
                        && room_y >= 0
                        && room_x < rms::ROOM_WIDTH as i32
                        && room_y < rms::ROOM_HEIGHT as i32
                    {
                        rooms[room_index].set_tile(room_x as u32, room_y as u32, 1);
                        request_paint(&event_subsystem);
                    }
                }
                Event::User { .. } => {
                    // HACK! The Rust-SDL2 API wants me to do something like:
                    // if event.is_user_event() {
                    //     let paint_event = event.as_user_event_type::<PaintEvent>().unwrap();
                    //     // do paint
                    // }
                    // But I know under the covers that it's just a User event, which works better
                    // in this match statement.
                    canvas.clear();
                    for y in 0..rms::ROOM_HEIGHT {
                        for x in 0..rms::ROOM_WIDTH {
                            let draw_rect = editor_view.rect(x, y);

                            let mut tile = rooms[room_index].get_tile(x, y);
                            if tile > 0 {
                                tile -= 1;
                                canvas
                                    .copy(&tiles_atlas[tile as usize], None, draw_rect)
                                    .unwrap();
                            }

                            match rooms[room_index].get_object_type(x, y) {
                                rms::ObjectType::Monster => {
                                    let monster_id = rooms[room_index].monster_id - 1;
                                    tile = monsters[monster_id as usize].gfx_id - 1;
                                    canvas
                                        .copy(&monsters_atlas[tile as usize], None, draw_rect)
                                        .unwrap();
                                }
                                rms::ObjectType::Object => {
                                    tile = rooms[room_index].get_object(x, y);
                                    if tile == 0 {
                                        continue;
                                    }
                                    // TODO: For some reason, I made it so I don't need to -1 here...
                                    // when I probably should since it's ambiguous whether 0 is the "no tile" sentinel or literally tile 0
                                    // tile -= 1;
                                    canvas
                                        .copy(&tiles_atlas[tile as usize], None, draw_rect)
                                        .unwrap();
                                }
                                _ => {}
                            }
                        }
                    }
                    canvas.present();
                }
                // Convert all other events into paint events (keep the screen fresh)
                // TODO: Make sure this works on stacking window managers which need constant
                // redraws of dirty areas (especially from other windows overlapping)
                _ => request_paint(&event_subsystem),
            }
        }
    }
}
