extern crate nalgebra as na;
extern crate find_folder;
extern crate gfx_device_gl;

use piston_window::*;


use self::gfx_device_gl::{Resources, CommandBuffer};



use object::Object;


use na::{ Vector2, Point2 };

pub type Vec2 = na::Vector2<f64>;

pub struct Game<T: character::CharacterCache> {
    player: Object,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool,
    sprint: bool,
    glyphs: T
}

use std;


impl <T : character::CharacterCache> Game <T  > {
    pub fn new(w: &mut PistonWindow, g: T) -> Game<T> {

        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let ship_sprite = assets.join("ship.gif");
        let ship_sprite = Texture::from_path(
                &mut w.factory,
                &ship_sprite,
                Flip::None,
                &TextureSettings::new())
                .unwrap();

        let mut p = Object::new();

        let sprites = get_ship_assets(w);
        p.set_sprites(sprites);

        Game { player: p, up_d: false, down_d: false, left_d: false, right_d: false, glyphs: g, sprint: false }
    }




    pub fn on_update(&mut self, upd: UpdateArgs) {

        let acceleration = if self.sprint { 2.0 } else { 1.0 };

        if self.up_d {
            self.player.vel += Vec2::new(self.player.rot.cos(), self.player.rot.sin()) * acceleration;
        }
        if self.down_d {
            self.player.vel -= Vec2::new(self.player.rot.cos(), self.player.rot.sin()) * acceleration;
        }
        if self.left_d {
            self.player.rot -= 0.01 * acceleration;
        }
        if self.right_d {
            self.player.rot += 0.01 * acceleration;
        }

        self.player.fwd(upd.dt);
    }
    pub fn on_draw<W, E>(&mut self, ren: RenderArgs, w: &mut PistonWindow<W>, e: &E)
    where W: Window, W::Event: GenericEvent, E: GenericEvent,
          T: character::CharacterCache< Texture = Texture<gfx_device_gl::Resources>>
    {

        // let glyphs =
        //        &mut Glyphs::new(font, factory).unwrap() as
        //        &mut character::CharacterCache< Texture = Texture<gfx_device_gl::Resources> >
        //    ;


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

            text(red, 12, &format!("Sprint: {}", self.sprint), &mut self.glyphs, transform, graphics)

        });
    }
    fn dir_changed(&mut self) {
        self.player.currSprite = match (self.left_d, self.right_d, self.sprint) {
            (true, false, true) => 0,
            (true, false, false) => 1,
            (true, true, _) => 2,
            (false, false, _) => 2,
            (false, true, false) => 3,
            (false, true, true) => 4
        }
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
                        self.sprint = true;
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
                        self.sprint = false;
                        true
                    }
                    _ => false
                }
            }
            _ => false
        };

        if did_change {
            self.dir_changed();
        }

    }
}



fn get_ship_assets(w: &mut PistonWindow) -> Vec<Texture<Resources>> {
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
