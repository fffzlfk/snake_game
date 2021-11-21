use opengl_graphics::GlGraphics;
use piston::input::*;

use super::food::Food;
use super::snake::Snake;

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
    square_width: u32,
    size: (u32, u32),
    food: Food,
    just_eaten: bool,
    score: u64,
}

impl Game {
    pub fn new(
        gl: GlGraphics,
        snake: Snake,
        square_width: u32,
        size: (u32, u32),
        food: Food,
    ) -> Game {
        Game {
            gl,
            snake,
            square_width,
            size,
            food,
            just_eaten: false,
            score: 0_u64,
        }
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }

    pub fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, args);
        self.food.render(&mut self.gl, args, self.square_width);
    }

    pub fn update(&mut self) -> bool {
        if !self.snake.update(self.just_eaten, self.size.1, self.size.0) {
            return false;
        }

        self.just_eaten = self.food.update(&self.snake);
        if self.just_eaten {
            self.score += 1;
            use rand::thread_rng;
            use rand::Rng;

            let mut r = thread_rng();
            loop {
                let new_x = r.gen_range(0..self.size.0);
                let new_y = r.gen_range(0..self.size.1);

                if !self.snake.is_collide(new_x, new_y) {
                    self.food = Food::new(new_x, new_y);
                    break;
                }
            }
        }
        true
    }

    pub fn pressed(&mut self, btn: &Button) {
        self.snake.pressed(btn);
    }
}
