extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::Rng;

use std::collections::VecDeque;

mod colors {
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Apple {
    pub body: VecDeque<(u32, u32)>,
}

pub struct Snek {
    pub body: VecDeque<(u32, u32)>,
    pub dir: Direction,
    pub is_alive: bool,
}

pub struct Game {
    pub gl: GlGraphics,
    pub snek: Snek,
    pub apple: Apple,
    pub is_running: bool,
    pub points: u32,
}

impl Apple {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let square: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            square
                .into_iter()
                .for_each(|square| graphics::rectangle(colors::WHITE, square, transform, gl));
        });
    }

    pub fn consume(&mut self, snek: &Snek) {
        self.body.pop_back();

        let (retx, rety) = loop {
            let newx = rand::thread_rng().gen_range(0, 32);
            let newy = rand::thread_rng().gen_range(0, 24);

            if !snek.body.contains(&(newx, newy)) {
                break (newx, newy);
            }
        };
        self.body.push_front((retx, rety));
    }
}

impl Snek {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(colors::RED, square, transform, gl));
        });
    }

    pub fn update(&mut self, apple: &mut Apple, points: &mut u32) {
        let mut new_head = (*self.body.front().expect("Snek has no body :(")).clone();
        let head = self.body.front().unwrap();

        match self.dir {
            Direction::Up => {
                if head.1 == 0 {
                    new_head.1 = 24;
                } else {
                    new_head.1 -= 1;
                }
            }
            Direction::Down => {
                if head.1 == 24 {
                    new_head.1 = 0;
                } else {
                    new_head.1 += 1;
                }
            }
            Direction::Left => {
                if head.0 == 0 {
                    new_head.0 = 32;
                } else {
                    new_head.0 -= 1;
                }
            }
            Direction::Right => {
                if head.0 == 32 {
                    new_head.0 = 0;
                } else {
                    new_head.0 += 1;
                }
            }
        }

        if self.body.contains(&(new_head.0, new_head.1)) {
            self.is_alive = false;
        }

        if apple.body.contains(&(new_head.0, new_head.1)) {
            apple.consume(&self);
            *points += 100;

            if *points % 300 == 0 {
                self.grow();
            }
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }

    fn grow(&mut self) {
        let mut extra_head = (*self.body.front().unwrap()).clone();
        let head = self.body.front().unwrap();

        match self.dir {
            Direction::Up => {
                if head.1 == 0 {
                    extra_head.1 = 24;
                } else {
                    extra_head.1 -= 1;
                }
            }
            Direction::Down => {
                if head.1 == 24 {
                    extra_head.1 = 0;
                } else {
                    extra_head.1 += 1;
                }
            }
            Direction::Left => {
                if head.0 == 0 {
                    extra_head.0 = 32;
                } else {
                    extra_head.0 -= 1;
                }
            }
            Direction::Right => {
                if head.0 == 32 {
                    extra_head.0 = 0;
                } else {
                    extra_head.0 += 1;
                }
            }
        }
        self.body.push_back(extra_head);
    }
}

impl Game {
    pub fn render(&mut self, arg: &RenderArgs) {
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(colors::BLACK, gl);
        });

        self.apple.render(&mut self.gl, arg);
        self.snek.render(&mut self.gl, arg);
    }

    pub fn update(&mut self) {
        self.snek.update(&mut self.apple, &mut self.points);
        if self.snek.is_alive == false {
            self.is_running = false;
        }
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

        match btn {
            &Button::Keyboard(Key::Escape) => self.is_running = false,
            _ => (),
        }
    }
}
