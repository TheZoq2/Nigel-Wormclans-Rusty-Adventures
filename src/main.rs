use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("duns ballistic demo", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 50, 80));
    canvas.clear();
    canvas.present();

    let dt = 1. / 60.;
    let mut now = Instant::now();
    let mut accumulated_time = 0.;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                }
                Event::KeyDown { keycode: Some(kc), .. } => {
                    match kc {
                        Keycode::Right => {
                            // TODO: Handle input
                        }
                        Keycode::Left => {
                            // TODO: Handle input
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let (screen_w, screen_h) = canvas.output_size().unwrap();

        let new_now = Instant::now();
        accumulated_time += new_now.duration_since(now).as_secs_f32();
        now = new_now;

        while accumulated_time >= dt {
            accumulated_time -= dt;

            // TODO: Run game loop
        }

        canvas.set_draw_color(Color::RGB(0, 50, 80));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 80));
        // TODO: Draw game
        canvas.present();
    }
}
