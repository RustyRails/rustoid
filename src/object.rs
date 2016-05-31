use piston_window::*;

extern crate gfx_device_gl;
extern crate find_folder;
extern crate gfx_graphics;
extern crate gfx;
extern crate nalgebra as na;


use self::gfx_device_gl::{Resources, CommandBuffer};
use self::gfx_graphics::GfxGraphics;

use na::{ Vector2, Point2 };

pub type Vec2 = na::Vector2<f64>;
pub type Pnt2 = Point2<f64>;

pub struct Object {
    pub loc: Pnt2,
    pub rot: f64,
    pub vel: Vec2,
    pub sprites: Vec<Texture<Resources>>,
    pub currSprite: usize
}

#[allow(dead_code)]
impl Object {
    pub fn new() -> Object {
        Object { loc: Pnt2::new(0.0, 0.0), rot: 0.0, vel: Vec2::new(0.0, 0.0), sprites: Vec::new(), currSprite: 0 }
    }

    pub fn fwd(&mut self, d: f64) {
        self.loc += self.vel * d;
    }

    pub fn mov(&mut self, x: f64, y: f64) {
        self.loc.x += x;
        self.loc.y += y;
    }

    pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        let square = rectangle::square(0.0, 0.0, 100.0);
        let red = [1.0, 0.0, 0.0, 1.0];

        match self.sprites.get(self.currSprite) {
            Some(sprite) => {
                let (x, y) = sprite.get_size();
                let (x, y) = ((x as f64) / -2.0, (y as f64) / -2.0);

                image(sprite, view
                    .trans(self.loc.x, self.loc.y)
                    .rot_rad(self.rot)
                    .trans(x, y)
                    , g);

                //rectangle(red, rectangle::square())
            }
            _ => {}
        }
    }

    pub fn set_sprites(&mut self, sprites: Vec<Texture<Resources>>) {
        self.sprites = sprites;
    }
}
