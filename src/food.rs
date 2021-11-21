use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::snake::Snake;

pub struct Food {
    x: u32,
    y: u32,
}

impl Food {
    pub fn new(x: u32, y: u32) -> Food {
        Food { x, y }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl)
        });
    }

    pub fn update(&mut self, s: &Snake) -> bool {
        let front = s.get_front().unwrap();

        if front.0 == self.x && front.1 == self.y {
            true
        } else {
            false
        }
    }
}
