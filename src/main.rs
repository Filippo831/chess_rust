//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod fen_functions;
mod gen_board;
mod movements;
mod vis_board;

use gen_board::generate_board;
use ggez::{
    conf::WindowMode,
    event,
    glam::*,
    graphics::{self, Color, DrawParam},
    mint::Point2,
    Context, GameResult,
};
use movements::get_piece;

use crate::vis_board::board_to_vis;

const SCREEN_SIZE: f32 = 400.0;

struct MainState {
    board_vec: Vec<graphics::Mesh>,
    board_test: Vec<Vec<char>>,
    pieces_vec: Vec<(graphics::Text, DrawParam)>,
    fen: String,
    moving_piece: movements::MovingStruct,
}

impl MainState {
    fn create_board(ctx: &mut Context) -> GameResult<MainState> {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR".to_string();
        let board_test: Vec<Vec<char>> = fen_functions::from_fen(&fen);
        let board_vec = gen_board::generate_board(SCREEN_SIZE, ctx);
        let pieces_vec = vis_board::board_to_vis(&board_test, ctx, SCREEN_SIZE);
        let moving_piece = movements::MovingStruct::new(false, '0');
        Ok(MainState {
            board_vec,
            board_test,
            pieces_vec,
            fen,
            moving_piece,
        })
    }
}
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mouse_position: Point2<f32> = _ctx.mouse.position();
        let is_clicked: bool = _ctx.mouse.button_just_pressed(event::MouseButton::Left);
        if is_clicked {
            if !self.moving_piece.fetch_is_moving() {
                movements::get_piece(
                    mouse_position,
                    &mut self.board_test,
                    SCREEN_SIZE / 8.0,
                    &mut self.moving_piece,
                );
                self.fen = fen_functions::to_fen(&self.board_test);
                self.board_vec = gen_board::generate_board(SCREEN_SIZE, _ctx);
                self.pieces_vec = vis_board::board_to_vis(&self.board_test, _ctx, SCREEN_SIZE);
            } else {
                movements::put_piece(
                    mouse_position,
                    &mut self.board_test,
                    SCREEN_SIZE / 8.0,
                    &mut self.moving_piece,
                );
                self.fen = fen_functions::to_fen(&self.board_test);
                self.board_vec = gen_board::generate_board(SCREEN_SIZE, _ctx);
                self.pieces_vec = vis_board::board_to_vis(&self.board_test, _ctx, SCREEN_SIZE);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for sq in &self.board_vec {
            canvas.draw(sq, Vec2::new(0.0, 0.0));
        }
        for piece in &self.pieces_vec {
            canvas.draw(&piece.0, piece.1);
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
        .add_resource_path("/home/filippo/Desktop/projects/chess_rust/sprites")
        .default_conf(conf)
        .window_mode(window_mode);

    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::create_board(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
