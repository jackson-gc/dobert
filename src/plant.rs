use std::io::Result;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crossterm::style::Stylize;

use crate::utils::draw::{Renderer, Point, Token};

const STEM_COUNT:usize = 12;

pub fn plant(renderer: &mut Renderer, rng_seed: u64, nut: &mut Point) -> Result<()> {
    let mut rng = ChaCha8Rng::seed_from_u64(rng_seed);
    let window_size = renderer.get_window_ctx();

    nut.x = window_size.0 / 2;
    nut.y = window_size.1 - (window_size.1 / 8) - 1;
    renderer.draw_tile('#'.red(), nut)?;
    let _all_tips: Vec<Point> = draw_branch(renderer, &mut rng, window_size);
    Ok(()) 
}



fn draw_branch(renderer: &mut Renderer, rng: &mut ChaCha8Rng, window_size: (u16, u16)) -> Vec::<Point> {
    
    let tip_mat: Token = '%'.dark_green();
    
    let mut all_tips = Vec::<Point>::new();
    let max_x = window_size.0 - 5;
    let max_y = window_size.1 - (window_size.1 / 6);
    for _ in 0..STEM_COUNT {
        let tip_pnt = Point {
            x: rng.random_range(5..max_x),
            y: rng.random_range(5..max_y)
        };
        let _ = renderer.draw_tile(tip_mat, &tip_pnt);
        all_tips.push(tip_pnt);
    };
    all_tips
}
