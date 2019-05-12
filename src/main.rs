use snek::{Direction, Game, Snek};

use std::collections::VecDeque;
use std::iter::FromIterator;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("snek", [600, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snek: Snek {
            body: VecDeque::from_iter((vec![(0.0, 0.0), (0.0, 1.0)]).into_iter()),
            dir: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
