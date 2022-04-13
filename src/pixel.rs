use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone, Copy, PartialEq)]
pub enum MaterialId {
    Empty = 0,
    Sand = 1,
    Water = 2,
    Acid = 3,
}

#[derive(Clone, Copy)]
pub struct Pixel {
    pub id: MaterialId,
    pub moving: bool,
    pub speed: i32,
    pub color: Color,
    pub point: Point,
}

impl Pixel {
    pub fn new(x: i32, y: i32, id: MaterialId) -> Pixel {
        match id {
            MaterialId::Sand => {
                return Pixel {
                    id,
                    speed: 2,
                    color: Color {
                        r: 194,
                        g: 178,
                        b: 128,
                        a: 255,
                    },
                    point: Point::new(x, y),
                    moving: false,
                };
            }
            MaterialId::Water => {
                return Pixel {
                    id,
                    speed: 2,
                    color: Color {
                        r: 0,
                        g: 0,
                        b: 255,
                        a: 125,
                    },
                    point: Point::new(x, y),
                    moving: false,
                };
            }
            _ => {}
        }
        return Pixel {
            id: MaterialId::Empty,
            moving: false,
            speed: 0,
            color: Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
            point: Point::new(0, 0),
        };
    }

    pub fn set_postion(&mut self, x: i32, y: i32) {
        self.point.x = x;
        self.point.y = y;
    }

    pub fn update(&mut self) {
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

pub fn get_pixel(point: Point, pixels: &mut Vec<Pixel>) -> Option<&mut Pixel> {
    for i in pixels {
        if point == i.point {
            return Some(i);
        }
    }
    return None;
}

pub fn get_pixel_id(point: Point, pixels: &Vec<Pixel>) -> MaterialId {
    for i in pixels {
        if point == i.point {
            return i.id;
        }
    }
    return MaterialId::Empty;
}

pub fn update_sand(x: i32, y: i32, pixels: &mut Vec<Pixel>) {
    let mut pixels_clone = pixels.clone();
    let mut pixel = get_pixel(Point::new(x, y), pixels).unwrap();
    if get_pixel_id(Point::new(x, y+1), &pixels_clone) == MaterialId::Empty {
        pixel.point.y += 1;
    }
    else if get_pixel_id(Point::new(x-1, y+1), &pixels_clone) == MaterialId::Empty {
        pixel.point.x -= 1;
        pixel.point.y += 1;
    } 
    else if get_pixel_id(Point::new(x+1, y+1), &pixels_clone) == MaterialId::Empty {
        pixel.point.x += 1;
        pixel.point.y += 1;
    }
}

pub fn update_water(x: i32, y: i32, pixels: &mut Vec<Pixel>) {
    let mut pixels_clone = pixels.clone();
    let mut pixel = get_pixel(Point::new(x, y), pixels).unwrap();
    if get_pixel_id(Point::new(x, y+1), &pixels_clone) == MaterialId::Empty {
        pixel.point.y += 1;
    }
    else if get_pixel_id(Point::new(x-1, y+1), &pixels_clone) == MaterialId::Empty {
        pixel.point.x -= 1;
        pixel.point.y += 1;
    } 
    else if get_pixel_id(Point::new(x+1, y+1), &pixels_clone) == MaterialId::Empty {
        pixel.point.x += 1;
        pixel.point.y += 1;
    }
    else if get_pixel_id(Point::new(x-1, y), &pixels_clone) == MaterialId::Empty {
        pixel.point.x -= 1;
    }
    else if get_pixel_id(Point::new(x+1, y), &pixels_clone) == MaterialId::Empty {
        pixel.point.x += 1;
    }
}
