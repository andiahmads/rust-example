use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().expect("SDL initilization failed");
    let video_subsytem = sdl_context
        .video()
        .expect("Couldn't get SDL video_subsytem");

    // setup windwos
    let window = video_subsytem
        .window("rust with sdl2 demo:", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("failed to create windows");

    // starting drawing
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to convert window into canvas");
    canvas.set_draw_color(Color::RGB(255, 0, 3));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}