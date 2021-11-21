mod food;
mod game;
mod snake;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::LinkedList;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use crate::food::Food;

fn main() {
    use game::Game;
    use snake::{Direction, Snake};
    let opengl = OpenGL::V4_3;

    const ROWS: u32 = 20;
    const COLS: u32 = 30;
    const SQUARE_WIDTH: u32 = 20;

    const WIDTH: u32 = COLS * SQUARE_WIDTH;
    const HEIGHT: u32 = ROWS * SQUARE_WIDTH;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(
        GlGraphics::new(opengl),
        Snake::new(
            LinkedList::from_iter((vec![(COLS / 2, ROWS / 2)]).into_iter()),
            Direction::Right,
            SQUARE_WIDTH,
        ),
        SQUARE_WIDTH,
        (COLS, ROWS),
        Food::new(1, 1),
    );

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_) = e.update_args() {
            if !game.update() {
                println!("Game Over");
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
    println!("Congratulations, your score was: {}", game.get_score());
}
