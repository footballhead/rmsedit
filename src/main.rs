mod cga;
mod crumb;
mod ega;

mod tests;

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

    let textures = ega::load_spritesheet("EGAPICS.PIC", &texture_creator);

    let mut debug_image_index = 0;
    let paint = &mut || {
        canvas.clear();
        canvas
            .copy(&textures[debug_image_index], None, None)
            .unwrap();
        canvas.present();
        debug_image_index = (debug_image_index + 1) % textures.len();
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
