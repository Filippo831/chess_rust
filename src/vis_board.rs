use ggez::{
    glam::{vec2, Vec2},
    graphics::{self, DrawParam, Drawable},
    Context,
};
use std::collections::HashMap;

pub fn board_to_vis(
    board: &Vec<Vec<char>>,
    ctx: &mut Context,
    tot_height: f32,
    canvas: &mut graphics::Canvas,
) {
    let data: Vec<(char, &str)> = vec![
        ('p', "./sprites/b_pawn_2x_ns.png"),
        ('r', "./sprites/b_rook_2x_ns.png"),
        ('n', "./sprites/b_knight_2x_ns.png"),
        ('q', "./sprites/b_queen_2x_ns.png"),
        ('k', "./sprites/b_king_2x_ns.png"),
        ('b', "./sprites/b_bishop_2x_ns.png"),
        ('P', "./sprites/w_pawn_2x_ns.png"),
        ('R', "./sprites/w_rook_2x_ns.png"),
        ('N', "./sprites/w_knight_2x_ns.png"),
        ('Q', "./sprites/w_queen_2x_ns.png"),
        ('K', "./sprites/w_king_2x_ns.png"),
        ('B', "./sprites/w_bishop_2x_ns.png"),
    ];
    let map: HashMap<char, &str> = HashMap::from_iter(data.into_iter());

    let im_size = tot_height / 8.0;

    for (y, line) in board.iter().enumerate() {
        for (x, e) in line.iter().enumerate() {
            if *e != '0' {
                dbg!(map.get(&e));
                let image = graphics::Image::from_path(ctx, map.get(&e).unwrap()).unwrap();
                let scale_factor: Vec2 = Vec2::new(
                    im_size / (image.height() as f32),
                    im_size / (image.height() as f32),
                );
                let draw_params = graphics::DrawParam::new().scale(scale_factor);
                image.draw(canvas, vec2(x as f32 * im_size, y as f32 * im_size));
            }
        }
    }
}
