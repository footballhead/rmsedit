use sdl2::rect::Rect;

mod cga;
mod crumb;
mod ega;
mod img;
mod monster;
mod rms;

mod tests;

fn main() {
    let rooms = rms::load_rooms("DUNGEON.RMS");
    let monsters = monster::load_monsters("PYMON.DAT");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Hello", 640, 480)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let textures = img::load_spritesheet("EGAPICS.PIC", &texture_creator);

    let mut debug_room_index: usize = 0;
    let paint = &mut || {
        canvas.clear();
        for y in 0..rms::ROOM_HEIGHT {
            for x in 0..rms::ROOM_WIDTH {
                let mut tile = rooms[debug_room_index].get_tile(x, y);
                if tile == 0 {
                    continue;
                }
                tile -= 1;

                canvas
                    .copy(
                        &textures[tile as usize],
                        None,
                        Rect::new(
                            (x * img::IMAGE_DIMENSION) as i32,
                            (y * img::IMAGE_DIMENSION) as i32,
                            img::IMAGE_DIMENSION,
                            img::IMAGE_DIMENSION,
                        ),
                    )
                    .unwrap();
            }
        }
        canvas.present();

        debug_room_index = (debug_room_index + 1) % rooms.len();
    };
    paint();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'mainloop: loop {
        for event in event_pump.wait_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'mainloop,
                _ => paint(),
            }
        }
    }
}
