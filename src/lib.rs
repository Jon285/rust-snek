extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::*;

use std::collections::VecDeque;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snek {
    pub body: VecDeque<(f32, f32)>,
    pub dir: Direction,
}

impl Snek {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square((x * 20.0) as f64, (y * 20.0) as f64, 20_f64)
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        });
    }

    pub fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snek has no body :(")).clone();

        match self.dir {
            Direction::Up => new_head.1 -= 1.0,
            Direction::Down => new_head.1 += 1.0,
            Direction::Left => new_head.0 -= 1.0,
            Direction::Right => new_head.0 += 1.0,
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }
}

pub struct Game {
    pub gl: GlGraphics,
    pub snek: Snek,
}

impl Game {
    pub fn render(&mut self, arg: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLACK, gl);
        });

        self.snek.render(&mut self.gl, arg);
    }

    pub fn update(&mut self) {
        self.snek.update();
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last = self.snek.dir.clone();

        self.snek.dir = match btn {
            &Button::Keyboard(Key::Up) if last != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last != Direction::Left => Direction::Right,
            _ => last,
        };
    }
}
