use std::collections::LinkedList;

use opengl_graphics::GlGraphics;
use piston::input::*;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct Snake {
    body: LinkedList<(u32, u32)>,
    dir: Direction,
    width: u32,
}

impl Snake {
    pub fn new(body: LinkedList<(u32, u32)>, dir: Direction, width: u32) -> Snake {
        Snake { body, dir, width }
    }

    pub fn get_front(&self) -> Option<&(u32, u32)> {
        self.body.front()
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares = self.body.iter().map(|&(x, y)| {
            graphics::rectangle::square(
                (x * self.width) as f64,
                (y * self.width) as f64,
                self.width as f64,
            )
        });

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.for_each(|square| graphics::rectangle(RED, square, transform, gl))
        })
    }

    pub fn update(&mut self, just_eaten: bool, rows: u32, cols: u32) -> bool {
        let mut new_head = self.body.front().expect("Snake has no body").clone();

        if (self.dir == Direction::Up && new_head.1 == 0)
            || (self.dir == Direction::Left && new_head.0 == 0)
            || (self.dir == Direction::Down && new_head.1 == rows - 1)
            || (self.dir == Direction::Right && new_head.0 == cols - 1)
        {
            return false;
        }

        match self.dir {
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
        }

        if !just_eaten {
            self.body.pop_back();
        }

        if self.is_collide(new_head.0, new_head.1) {
            return false;
        }

        self.body.push_front((new_head.0, new_head.1));

        true
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_dir = self.dir.clone();
        self.dir = match *btn {
            Button::Keyboard(Key::Up) if last_dir != Direction::Down => Direction::Up,
            Button::Keyboard(Key::Down) if last_dir != Direction::Up => Direction::Down,
            Button::Keyboard(Key::Left) if last_dir != Direction::Right => Direction::Left,
            Button::Keyboard(Key::Right) if last_dir != Direction::Left => Direction::Right,
            _ => last_dir,
        };
    }

    pub fn is_collide(&self, x: u32, y: u32) -> bool {
        self.body.iter().any(|p| x == p.0 && y == p.1)
    }
}
