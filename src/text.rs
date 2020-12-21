use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;

/// Stores results for rendering text to a drawable texture with conveniences for copying to a
/// canvas.
///
/// # Example
///
/// ```
/// // Setup
/// let sdl_context = sdl2::init().unwrap();
/// let video_subsystem = sdl_context.video().unwrap();
/// let window = video_subsystem
///     .window("Hello", 640, 480)
///     .position_centered()
///     .build()
///     .unwrap();
/// let mut canvas = window.into_canvas().build().unwrap();
/// let texture_creator = canvas.texture_creator();
/// let ttf_context = sdl2::ttf::init().unwrap();
/// let liberation_sans = ttf_context
///     .load_font(
///         "LiberationSans-Regular.ttf",
///         16,
///     )
///     .unwrap();
///
/// // Render text
/// let test_text = TextRendering::from_text(
///     "Hello world",
///     &Color::RGB(0xFF, 0xFF, 0xFF),
///     &liberation_sans,
///     &texture_creator,
/// );
///
/// // Show rendered text at (0, 0)
/// canvas
///     .copy(test_text.texture(), None, test_text.rect(0, 0))
///     .unwrap();
/// canvas.present();
/// ```
pub struct TextRendering<'r> {
    texture: Texture<'r>,
    width: u32,
    height: u32,
}

impl<'r> TextRendering<'r> {
    pub fn from_text<CanvasT>(
        text: &str,
        color: &Color,
        font: &Font,
        texture_creator: &'r TextureCreator<CanvasT>,
    ) -> TextRendering<'r> {
        let surface = font.render(text).blended(*color).unwrap();
        TextRendering {
            texture: surface.as_texture(texture_creator).unwrap(),
            width: surface.width(),
            height: surface.height(),
        }
    }

    pub fn rect(&self, x: i32, y: i32) -> Rect {
        Rect::new(x, y, self.width, self.height)
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}

/// Holds text and related info that can be easily rerendered at any time.
///
/// Basically a convenience wrapper around TextRendering that remembers colors, fonts, etc.
///
/// # Examples
///
/// ```
/// // Setup
/// let sdl_context = sdl2::init().unwrap();
/// let video_subsystem = sdl_context.video().unwrap();
/// let window = video_subsystem
///     .window("Hello", 640, 480)
///     .position_centered()
///     .build()
///     .unwrap();
/// let mut canvas = window.into_canvas().build().unwrap();
/// let texture_creator = canvas.texture_creator();
/// let ttf_context = sdl2::ttf::init().unwrap();
/// let liberation_sans = ttf_context
///     .load_font(
///         "LiberationSans-Regular.ttf",
///         16,
///     )
///     .unwrap();
///
/// // Render text
/// let mut text_field = TextLabel::new(
///     "Hello world",
///     &Color::RGB(0xFF, 0xFF, 0xFF),
///     &liberation_sans,
///     &texture_creator,
/// );
///
/// // Show "Hello world" at (0, 0)
/// canvas
///     .copy(text_field.texture(), None, text_field.rect(0, 0))
///     .unwrap();
/// canvas.present();
///
/// // Update + rerender text with original params
/// text_field.update("foobar");
///
/// // Show "foobar" at (0, 0)
/// canvas
///     .copy(text_field.texture(), None, text_field.rect(0, 0))
///     .unwrap();
/// canvas.present();
/// ```
pub struct TextLabel<'r, CanvasT> {
    rendering: TextRendering<'r>,
    color: Color,
    font: &'r Font<'r, 'r>,
    texture_creator: &'r TextureCreator<CanvasT>,
}

impl<'r, CanvasT> TextLabel<'r, CanvasT> {
    pub fn new(
        text: &str,
        color: Color,
        font: &'r Font,
        texture_creator: &'r TextureCreator<CanvasT>,
    ) -> TextLabel<'r, CanvasT> {
        TextLabel {
            rendering: TextRendering::from_text(text, &color, font, texture_creator),
            color: color,
            font: font,
            texture_creator: texture_creator,
        }
    }

    pub fn update(&mut self, text: &str) {
        self.rendering =
            TextRendering::from_text(text, &self.color, self.font, self.texture_creator);
    }

    pub fn rect(&self, x: i32, y: i32) -> Rect {
        self.rendering.rect(x, y)
    }

    pub fn texture(&self) -> &Texture {
        self.rendering.texture()
    }
}
