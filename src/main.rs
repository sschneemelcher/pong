extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("pong", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas()
        .present_vsync()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    std::process::exit(
        run(&mut canvas, &mut event_pump)
    );
} 


fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            println!("found opengl driver");
            return Some(index as u32);
        }
    }
    println!("did not find opengl driver");
    None
}


fn run(canvas: &mut sdl2::render::WindowCanvas, event_pump: &mut sdl2::EventPump) -> i32 {
    loop {

        canvas.set_draw_color(Color::RGB(0, 0 , 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => return 0,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rects(&[
            Rect::new(10, 200, 20, 200),
            Rect::new(770, 200, 20, 200),
            Rect::new(390, 140, 20, 20)
            ]).ok();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

