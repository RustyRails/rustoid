extern crate piston_window;
extern crate nalgebra as na;
extern crate find_folder;


use piston_window::*;

mod object;
mod game;

use object::Object;
use game::Game;


use na::{ Vector2, Point2 };

pub type Vec2 = na::Vector2<f64>;


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rustoid!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();

    let mut game = Game::new(&mut window, Glyphs::new(font, factory).unwrap() );

    while let Some(e) = window.next() {
        match e {
            Event::Update(upd) => {
                game.on_update(upd);
            }
            Event::Render(ren) => {
                game.on_draw(ren, &mut window, &e);
            }
            Event::Input(inp) => {
                game.on_input(inp);
            }
            _ => {
            }
        }
    }
}
