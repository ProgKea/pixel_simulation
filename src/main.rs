#![allow(unused)]

mod pixel;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::mouse;
use std::time::Duration;

extern crate sdl2; 

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("Pixel Simulation", 1200, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();
    let mut pixels: Vec<pixel::Pixel> = Vec::new();

    'running: loop {
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.clear();
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        for y in (0..canvas.window().drawable_size().1 as i32).rev() {
            for x in (0..canvas.window().drawable_size().0 as i32).rev() {
                let mat_id = pixel::get_pixel(Point::new(x, y), &pixels);
                match mat_id {
                    pixel::MaterialId::Sand => pixel::update_sand(x as u32, y as u32),
                    pixel::MaterialId::Water => pixel::update_water(x as u32, y as u32),
                    _ => {}
                }
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
