extern crate nalgebra as na;
extern crate find_folder;
extern crate gfx_device_gl;

use piston_window::*;


use self::gfx_device_gl::{Resources, CommandBuffer};



use ship::Ship;
use ship::ShipTurn;
use ship::ShipThrust;

use na::{ Vector2, Point2 };

pub type Vec2 = na::Vector2<f64>;

pub struct Game<T: character::CharacterCache> {
    player: Ship,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool, shift_d: bool,
    glyphs: T
}

use std;


impl <T : character::CharacterCache> Game <T  > {
    pub fn new(w: &mut PistonWindow, g: T) -> Game<T> {

        let mut p = Ship::new(w);

        Game { player: p, up_d: false, down_d: false, left_d: false, right_d: false, shift_d: false, glyphs: g }
    }

    pub fn on_update(&mut self, upd: UpdateArgs) {
        self.player.on_update(upd.dt);
    }

    pub fn on_draw<W, E>(&mut self, ren: RenderArgs, w: &mut PistonWindow<W>, e: &E)
    where W: Window, W::Event: GenericEvent, E: GenericEvent,
          T: character::CharacterCache< Texture = Texture<gfx_device_gl::Resources>>
    {

        w.draw_2d(e, |c, graphics| {

            let damp = 200.0;

            let r = (self.player.loc.x / damp).sin().abs() as f32;
            let g = (self.player.loc.y / damp).sin().abs() as f32;
            let b = ((self.player.loc.x + self.player.loc.y) / damp).sin().abs() as f32;

            clear([r, g, b, 1.0], graphics);

            let center = c.transform.trans((ren.width / 2) as f64, (ren.height / 2) as f64);
            self.player.render(graphics, center);


            let red = [1.0, 0.0, 0.0, 1.0];

            //rectangle(red, square, center.rot_rad(self.rotation).trans(-50.0, -50.0), graphics);
            let transform = c.transform.trans(10.0, 10.0);

            text(red, 12, &format!("loc: {}", self.player.loc), &mut self.glyphs, transform, graphics)

        });
    }

    fn controls_changed(&mut self) {
        self.player.boosting = self.shift_d;

        self.player.turning = match (self.left_d, self.right_d) {
            (true, false) => ShipTurn::Left,
            (true, true) => ShipTurn::None,
            (false, false) => ShipTurn::None,
            (false, true) => ShipTurn::Right
        };
        self.player.turning = match (self.left_d, self.right_d) {
            (true, false) => ShipTurn::Left,
            (true, true) => ShipTurn::None,
            (false, false) => ShipTurn::None,
            (false, true) => ShipTurn::Right
        };
        self.player.thrusting = match (self.up_d, self.down_d) {
            (true, false) => ShipThrust::Engaged,
            (true, true) => ShipThrust::None,
            (false, false) => ShipThrust::None,
            (false, true) => ShipThrust::Reverse
        };
    }

    pub fn on_input(&mut self, inp: Input) {
        let did_change = match inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = true;
                        true
                    },
                    Button::Keyboard(Key::Down) => {
                        self.down_d = true;
                        true
                    },
                    Button::Keyboard(Key::Left) => {
                        self.left_d = true;
                        true
                    },
                    Button::Keyboard(Key::Right) => {
                        self.right_d = true;
                        true
                    },
                    Button::Keyboard(Key::LShift) => {
                        self.shift_d = true;
                        true
                    },
                    _ => false
                }
            }
            Input::Release(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = false;
                        true
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = false;
                        true
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = false;
                        true
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = false;
                        true
                    }
                    Button::Keyboard(Key::LShift) => {
                        self.shift_d = false;
                        true
                    }
                    _ => false
                }
            }
            _ => false
        };

        if did_change {
            self.controls_changed();
        }

    }
}
