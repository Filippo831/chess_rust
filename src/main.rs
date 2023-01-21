//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod fen_functions;
mod gen_board;

use ggez::{
    conf::WindowMode,
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

const SCREEN_SIZE: f32 = 400.0;

struct MainState {
    board_vec: Vec<graphics::Mesh>,
}

impl MainState {
    fn create_board(ctx: &mut Context) -> GameResult<MainState> {
        let mut board_vec = vec![];
        board_vec = gen_board::generate_board(SCREEN_SIZE, ctx);
        Ok(MainState { board_vec })
    }
}
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for sq in &self.board_vec {
            canvas.draw(sq, Vec2::new(0.0, 0.0));
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    //set dimension
    let window_mode = WindowMode::default()
        .dimensions(SCREEN_SIZE, SCREEN_SIZE)
        .resizable(false);
    let mut conf = ggez::conf::Conf::new();

    //change window title
    conf.window_setup.title = "chesso".to_owned();
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .default_conf(conf)
        .window_mode(window_mode);

    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::create_board(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
