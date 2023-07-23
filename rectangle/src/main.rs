use std::thread::sleep;
use std::time::{Duration, SystemTime};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32,
) -> Option<Texture<'a>> {
    // we'll want handle failures outside of this function.
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                    TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
                }
                texture.clear();
            })
            .expect("failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsytem = sdl_context
        .video()
        .expect("Couldn't get SDL video subsystem");

    //paramter are: title,width,height
    let window = video_subsytem
        .window("Tetris", 800, 600)
        .position_centered()
        .build()
        .expect("failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("failed to get windows");

    // create var texture creator, for drawing object
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    // for texture size

    // we create texture size with 32x32 size.
    let green_squear = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Green,
        TEXTURE_SIZE,
    )
    .expect("Failed to create a texture green");

    let blue_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Blue,
        TEXTURE_SIZE,
    )
    .expect("failed to create a texture blue");

    let timer = SystemTime::now();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // IF we receive a 'quit' event or user press ESC
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // set fulfill our window with red
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        // we draw it
        canvas.clear();

        // the ractangle switch happens here:
        let display_green = match timer.elapsed() {
            Ok(elepased) => elepased.as_secs() % 2 == 0,
            Err(_) => true,
        };

        let square_texture = if display_green {
            &green_squear
        } else {
            &blue_square
        };

        canvas
            .copy(
                square_texture,
                None,
                // we copy it at the top-left of the window with a 32x32 size
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
            )
            .expect("couldn copy texture into window");
        canvas.present();

        // We sleep enough to get ~60 fps. If we don't call this,
        // the program will take
        // 100% of a CPU time.
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
