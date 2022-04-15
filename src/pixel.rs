use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const SCALE: i32 = 1;
pub const WIDTH: u32 = SCALE as u32 * 100;
pub const HEIGHT: u32 = SCALE as u32 * 50;

#[derive(Clone, Copy, PartialEq)]
pub enum MaterialId {
    Empty = 0,
    Sand = 1,
    Water = 2,
    Wood = 3,
}

#[derive(Clone, Copy)]
pub struct Pixel {
    pub id: MaterialId,
    pub color: Color,
    pub point: Point,
}

impl Pixel {
    pub fn new(x: i32, y: i32, id: MaterialId) -> Pixel {
        match id {
            MaterialId::Sand => {
                return Pixel {
                    id,
                    color: Color {
                        r: 194,
                        g: 178,
                        b: 128,
                        a: 255,
                    },
                    point: Point::new(x, y),
                };
            }
            MaterialId::Water => {
                return Pixel {
                    id,
                    color: Color {
                        r: 0,
                        g: 0,
                        b: 255,
                        a: 125,
                    },
                    point: Point::new(x, y),
                };
            }
            MaterialId::Wood => {
                return Pixel {
                    id,
                    color: Color {
                        r: 43,
                        g: 29,
                        b: 20,
                        a: 255,
                    },
                    point: Point::new(x, y),
                }
            }
            _ => {}
        }
        return Pixel {
            id: MaterialId::Empty,
            color: Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
            point: Point::new(0, 0),
        };
    }
}

pub trait RenderPixel {
    fn draw_pixel(&mut self, pixel: &mut Pixel);
}

impl RenderPixel for Canvas<Window> {
    fn draw_pixel(&mut self, pixel: &mut Pixel) {
        self.set_draw_color(pixel.color);
        self.draw_point(pixel.point);
        if pixel.point.y > HEIGHT as i32 / SCALE {
            pixel.point.y = HEIGHT as i32 / SCALE - 1;
        }
    }
}

pub fn circle_vector(centre_x: i32, centre_y: i32, radius: i32, pixel_vector: &mut Vec<Pixel>, index_id: MaterialId) {
   let diameter: i32 = radius + 2;

   let mut x = (radius - 1);
   let mut y = 0;
   let mut tx = 1;
   let mut ty = 1;
   let mut error = (tx - diameter);

   while (x >= y)
   {
      //  Each of the following renders an octant of the circle
      pixel_vector.push(Pixel::new(centre_x + x, centre_y - y, index_id));
      pixel_vector.push(Pixel::new(centre_x + x, centre_y + y, index_id));
      pixel_vector.push(Pixel::new(centre_x - x, centre_y - y, index_id));
      pixel_vector.push(Pixel::new(centre_x - x, centre_y + y, index_id));
      pixel_vector.push(Pixel::new(centre_x + y, centre_y - x, index_id));
      pixel_vector.push(Pixel::new(centre_x + y, centre_y + x, index_id));
      pixel_vector.push(Pixel::new(centre_x - y, centre_y - x, index_id));
      pixel_vector.push(Pixel::new(centre_x - y, centre_y + x, index_id));

      if (error <= 0)
      {
         { y += 1; y };
         error += ty;
         ty += 2;
      }

      if (error > 0)
      {
         --x;
         tx += 2;
         error += (tx - diameter);
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

    let pixel_below = get_pixel_id(Point::new(x, y + 1), &pixels_clone);
    let pixel_below_left = get_pixel_id(Point::new(x - 1, y + 1), &pixels_clone);
    let pixel_below_right = get_pixel_id(Point::new(x + 1, y + 1), &pixels_clone);

    if pixel_below == MaterialId::Empty {
        pixel.point.y += 1;
    } else if pixel_below_left == MaterialId::Empty {
        if pixel.point.x > 0 {
            pixel.point.x -= 1;
            pixel.point.y += 1;
        }
    } else if pixel_below_right == MaterialId::Empty {
        if pixel.point.x < WIDTH as i32 / SCALE {
            pixel.point.x += 1;
            pixel.point.y += 1;
        }
    }
}

pub fn update_water(x: i32, y: i32, pixels: &mut Vec<Pixel>) {
    let mut pixels_clone = pixels.clone();
    let mut pixel = get_pixel(Point::new(x, y), pixels).unwrap();

    let pixel_below = get_pixel_id(Point::new(x, y + 1), &pixels_clone);
    let pixel_below_left = get_pixel_id(Point::new(x - 1, y + 1), &pixels_clone);
    let pixel_below_right = get_pixel_id(Point::new(x + 1, y + 1), &pixels_clone);
    let pixel_left = get_pixel_id(Point::new(x - 1, y), &pixels_clone);
    let pixel_right = get_pixel_id(Point::new(x + 1, y), &pixels_clone);

    if pixel_below == MaterialId::Empty {
        pixel.point.y += 1;
    } else if pixel_below_left == MaterialId::Empty {
        if pixel.point.x > 0 {
            pixel.point.x -= 1;
            pixel.point.y += 1;
        }
    } else if pixel_below_right == MaterialId::Empty {
        if pixel.point.x < WIDTH as i32 / SCALE {
            pixel.point.x += 1;
            pixel.point.y += 1;
        }
    } else if pixel_left == MaterialId::Empty {
        if pixel.point.x > 0 {
            pixel.point.x -= 1;
        }
    } else if pixel_right == MaterialId::Empty {
        if pixel.point.x < WIDTH as i32 / SCALE {
            pixel.point.x += 1;
        }
    }
}
