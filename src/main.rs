#![allow(unused)]

mod pixel;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Pixel Simulation", 50, 50)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();
    let mut pixels: Vec<pixel::Pixel> = Vec::new();
    let mut index_id: pixel::MaterialId = pixel::MaterialId::Sand;

    'running: loop {
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.clear();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    index_id = pixel::MaterialId::Sand;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    index_id = pixel::MaterialId::Water;
                }
                _ => {}
            }
        }

        if events
            .mouse_state()
            .is_mouse_button_pressed(mouse::MouseButton::Left)
        {
            let state = events.mouse_state();
            let &mut new_pixel = &mut pixel::Pixel::new(
                state.x() / canvas.scale().0 as i32,
                state.y() / canvas.scale().0 as i32,
                index_id,
            );
            pixels.push(new_pixel);
        }

        for y in (0..canvas.window().drawable_size().1 as i32).rev() {
            for x in (0..canvas.window().drawable_size().0 as i32).rev() {
                let mat_id = pixel::get_pixel_id(Point::new(x, y), &pixels);
                match mat_id {
                    pixel::MaterialId::Sand => pixel::update_sand(x, y, &mut pixels),
                    pixel::MaterialId::Water => pixel::update_water(x, y, &mut pixels),
                    _ => {}
                }
            }
        }

        for mut i in &mut pixels {
            pixel::RenderPixel::draw_pixel(&mut canvas, &mut i);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
