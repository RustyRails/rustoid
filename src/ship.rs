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

pub enum ShipTurn {
    None, Left, Right
}

pub enum ShipThrust {
    None, Reverse, Engaged
}


pub struct Ship {
    pub loc: Pnt2,
    pub rot: f64,
    pub vel: Vec2,
    frames: Vec<Texture<Resources>>,
    exhausts: Vec<Texture<Resources>>,
    pub boosting: bool,
    pub turning: ShipTurn,
    pub thrusting: ShipThrust
}


impl Ship {
    pub fn new(w: &mut PistonWindow) -> Ship {
        Ship {
            loc: Pnt2::new(0.0, 0.0),
            rot: 0.0,
            vel: Vec2::new(0.0, 0.0),
            frames: get_ship_frames(w),
            exhausts: get_ship_exhausts(w),
            boosting: false,
            turning: ShipTurn::None,
            thrusting: ShipThrust::None
        }
    }

    pub fn on_update(&mut self, dt: f64) {
        let mult = if self.boosting { 2.0 } else { 1.0 };

        self.rot += dt * match self.turning {
            ShipTurn::None => 0.0,
            ShipTurn::Left => -0.01 * mult,
            ShipTurn::Right => 0.01 * mult
        };

        self.vel += dt * match self.thrusting {
            ShipThrust::None => Vec2::new(0.0, 0.0),
            ShipThrust::Engaged => Vec2::new(self.rot.cos(), self.rot.sin()) * mult,
            ShipThrust::Reverse => Vec2::new(self.rot.cos(), self.rot.sin()) * -mult,
        };

    }

    fn get_curr_frame(&self) -> usize {
        match (self.turning, self.boosting) {
            (ShipTurn::Left, true) => 0,
            (ShipTurn::Left, false) => 1,
            (ShipTurn::None, _) => 2,
            (ShipTurn::Right, false) => 3,
            (ShipTurn::Right, true) => 4,
        }
    }

    pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        let square = rectangle::square(0.0, 0.0, 100.0);
        let red = [1.0, 0.0, 0.0, 1.0];

        match self.frames.get(self.get_curr_frame()) {
            Some(frame) => {
                let (x, y) = frame.get_size();
                let (x, y) = ((x as f64) / -2.0, (y as f64) / -2.0);

                image(frame, view
                    .trans(self.loc.x, self.loc.y)
                    .rot_rad(self.rot)
                    .trans(x, y)
                    , g);

                //rectangle(red, rectangle::square())
            }
            _ => {}
        }
    }

}

fn get_ship_frames(w: &mut PistonWindow) -> Vec<Texture<Resources>> {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();


    vec!["l1", "l2", "m", "r1", "r2"].iter().map(|x| {
        let spritename = format!("Player/player_b_{}.png", x);
        //println!("Trying with {}", spritename);
        let ship_sprite = assets.join(spritename);
        Texture::from_path(
                &mut w.factory,
                &ship_sprite,
                Flip::None,
                &TextureSettings::new())
                .unwrap()
    }).collect()

}

fn get_ship_exhausts(w: &mut PistonWindow) -> Vec<Texture<Resources>> {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    vec!["01", "02", "03", "04", "05"].iter().map(|x| {
        let spritename = format!("Player/exhaust_{}.png", x);
        let ship_sprite = assets.join(spritename);
        Texture::from_path(
                &mut w.factory,
                &ship_sprite,
                Flip::None,
                &TextureSettings::new())
                .unwrap()
    }).collect()

}
