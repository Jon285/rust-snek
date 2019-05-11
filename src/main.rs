extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snek {
    x: f32,
    y: f32,
    dir: Direction,
}

impl Snek {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square =
            graphics::rectangle::square((self.x * 20.0) as f64, (self.y * 20.0) as f64, 20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(red, square, transform, gl);
        });
    }

    fn update(&mut self) {
        match self.dir {
            Direction::Up => self.y -= 1.0,
            Direction::Down => self.y += 1.0,
            Direction::Left => self.x -= 1.0,
            Direction::Right => self.x += 1.0,
        }
    }
}

struct Game {
    gl: GlGraphics,
    snek: Snek,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(black, gl);
        });

        self.snek.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snek.update();
    }

    fn pressed(&mut self, btn: &Button) {
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
            x: 0.0,
            y: 0.0,
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
