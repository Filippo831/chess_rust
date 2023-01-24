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
) -> Vec<(graphics::Text, DrawParam)> {
    let mut pieces_vec: Vec<(graphics::Text, DrawParam)> = vec![];

    let data: Vec<(char, &str)> = vec![
        ('p', "/b_pawn_2x_ns.png"),
        ('r', "/b_rook_2x_ns.png"),
        ('n', "/b_knight_2x_ns.png"),
        ('q', "/b_queen_2x_ns.png"),
        ('k', "/b_king_2x_ns.png"),
        ('b', "/b_bishop_2x_ns.png"),
        ('P', "/w_pawn_2x_ns.png"),
        ('R', "/w_rook_2x_ns.png"),
        ('N', "/w_knight_2x_ns.png"),
        ('Q', "/w_queen_2x_ns.png"),
        ('K', "/w_king_2x_ns.png"),
        ('B', "/w_bishop_2x_ns.png"),
    ];
    let map: HashMap<char, &str> = HashMap::from_iter(data.into_iter());

    let im_size = tot_height / 8.0;

    for (y, line) in board.iter().enumerate() {
        for (x, e) in line.iter().enumerate() {
            if *e != '0' {
                //let image = graphics::Image::from_path(ctx, map.get(&e).unwrap()).unwrap();
                //let scale_factor: Vec2 = Vec2::new(
                //im_size / (image.height() as f32),
                //im_size / (image.height() as f32),
                //);
                //let draw_params = graphics::DrawParam::new()
                //.scale(scale_factor)
                //.dest(vec2(x as f32 * im_size, y as f32 * im_size));
                //pieces_vec.push((image, draw_params));
                //image.draw(canvas, draw_params);
                let word = graphics::Text::new(*e);
                let word_params: graphics::DrawParam = graphics::DrawParam::new().dest(vec2(
                    (x as f32 * im_size) + im_size / 2.0,
                    (y as f32 * im_size) + im_size / 2.0,
                ));
                pieces_vec.push((word, word_params));
            }
        }
    }
    return pieces_vec;
}
