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
    let sdl_context     = sdl2::init().unwrap();
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
    exit(run(&mut canvas, &mut event_pump));
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
   
    let mut player2    = Rect::new(10, 200, 20, 200);
    let mut player1    = Rect::new(770, 200, 20, 200);
    let mut up         = false;
    let mut down       = false;
    let mut ball       = Rect::new(390, 140, 20, 20);
    let mut ball_dir_x = -1.0f32;
    let mut ball_dir_y = -1.0f32;

    ball_dir_x += rand::random::<f32>();
    ball_dir_x += rand::random::<f32>();
    ball_dir_y += rand::random::<f32>();
    ball_dir_y += rand::random::<f32>();

    if ball_dir_x * ball_dir_x < 0.0004 {ball_dir_x = 0.2}; 
    if ball_dir_y * ball_dir_x < 0.0004 {ball_dir_y = 0.2}; 

    loop {
        canvas.set_draw_color(Color::RGB(0, 0 , 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..}|
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return 0;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. }     => up   = true,
                Event::KeyDown { keycode: Some(Keycode::Down), .. }   => down = true,
                Event::KeyUp   { keycode: Some(Keycode::Up), .. }     => up   = false,
                Event::KeyUp   { keycode: Some(Keycode::Down), .. }   => down = false,
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

        ball.set_x(ball.x() + (ball_dir_x * 50.0) as i32);
        ball.set_y(ball.y() + (ball_dir_y * 50.0) as i32);

        if ball.right() >= player1.left() 
            && ball.left() <= player1.right()
                && ball.top() >= player1.top() 
                    && ball.bottom() <= player1.bottom() {
            ball.set_x(ball.x() - (ball.right() - player1.left()));
            ball_dir_x = -(ball_dir_x);
        }
        if ball.left() <= player2.right() 
            && ball.right() >= player2.left()
                && ball.top() >= player2.top() 
                    && ball.bottom() <= player2.bottom() {
            ball.set_x(ball.x() - (ball.left() - player2.right()));
            ball_dir_x = -(ball_dir_x);
        }
 

        if ball.right() > 800 || ball.left() < 0 {
            ball.set_x(390);
            ball.set_y(140);
            ball_dir_x = -1.0;
            ball_dir_y = -1.0;
            ball_dir_x += rand::random::<f32>();
            ball_dir_x += rand::random::<f32>();
            ball_dir_y += rand::random::<f32>();
            ball_dir_y += rand::random::<f32>();
            if ball_dir_x * ball_dir_x < 0.0004 {ball_dir_x = 0.2}; 
            if ball_dir_y * ball_dir_x < 0.0004 {ball_dir_y = 0.2}; 
        }
        if ball.top() > 600 {
            ball.set_y(ball.y() - (ball.bottom() - 600));
            ball_dir_y = -(ball_dir_y);
        }
        if ball.top() < 0 {
            ball.set_y(ball.y() - ball.top());
            ball_dir_y = -(ball_dir_y);
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

