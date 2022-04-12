use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::mouse;
use std::time::Duration;

#[derive(Clone, Copy)]
pub enum MaterialId {
    Empty = -2,
    Sand = 1,
    Water = 2,
}

#[derive(Clone, Copy)]
pub struct Pixel {
    id: MaterialId,
    updated: bool,
    speed: i32,
    color: Color,
    point: Point,
}

impl Pixel {
    fn new(color: Color, speed: i32, id: MaterialId) -> Pixel {
        return Pixel { id, updated: false, speed, color, point: Point::new(0, 0) }
    }
    fn update(&mut self) {
        self.point.y += self.speed;
    }
}

pub trait RenderPixel {
    fn draw_pixel(&mut self, pixel: &mut Pixel);
}

impl RenderPixel for Canvas<Window> {
    fn draw_pixel(&mut self, pixel: &mut Pixel) {
        self.set_draw_color(pixel.color);
        self.draw_point(pixel.point);
        if pixel.point.y >= self.window().drawable_size().1 as i32 - 1 {
            pixel.point.y = self.window().drawable_size().1 as i32 - 1;
        }
    }
}

pub fn update_sand(x: u32, y: u32) {
    todo!();
}

pub fn update_water(x: u32, y: u32) {
    todo!();
}

pub fn get_pixel(point: Point, pixels: &Vec<Pixel>) -> MaterialId {
    for i in pixels {
        if point == i.point {
            return i.id;
        }
    }
    return MaterialId::Empty;
}
