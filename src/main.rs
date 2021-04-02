extern crate sdl2; 
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use std::time::Duration;
use std::process::exit;
 
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

    exit(
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


fn run(canvas: &mut WindowCanvas, event_pump: &mut EventPump) -> i32 {
    let mut player2 = Rect::new(10, 200, 20, 200);
    let mut player1 = Rect::new(770, 200, 20, 200);
    let mut up = false;
    let mut down = false;
    let mut ball = Rect::new(390, 140, 20, 20);
    loop {

        canvas.set_draw_color(Color::RGB(0, 0 , 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..}|
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return 0;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => up = true,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => down = true,
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => up = false,
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => down = false,
                _ => {}
            }
        }

        if up && player1.top() > 0 {
            player1.set_y(player1.top() - 20);
        }

        if down && player1.top() < 400 {
            player1.set_y(player1.top() + 20);
        }


        if rand::random() {
                if player2.top() > 0 {
                    player2.set_y(player2.top() - 20);
                }
        } else {
                if player2.top() < 400 { 
                    player2.set_y(player2.top() + 20);
                }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rects(&[
            player1,
            player2, 
            ball,
            ]).ok();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

