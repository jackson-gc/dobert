use std::io::Result;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crossterm::style::Stylize;

use crate::utils::draw_utils::{Renderer, Point, Token};

const STEM_COUNT:usize = 16;

pub fn plant(renderer: &mut Renderer, seed: u64) -> Result<()> {
    let window_size = renderer.get_window_ctx();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut all_tips = Vec::<Point>::new();
    for _ in 0..STEM_COUNT {
        let tip_pnt = Point {
            x: rng.random_range(5..window_size.0 - 5),
            y: rng.random_range(5..(window_size.1 - (window_size.1 / 4)))
        };
        all_tips.push(tip_pnt);
    };


    draw_branch(renderer, &mut rng, 16)?;
    Ok(()) 
}



fn draw_branch(renderer: &mut Renderer, rng: &mut ChaCha8Rng, stems: u16) -> Result<()> {
    let window_size = renderer.get_window_ctx();
    let tip_mat: Token = '%'.dark_green();
    
    let mut all_tips = Vec::<Point>::new();
    for _ in 0..stems {
        let tip_pnt = Point {
            x: rng.random_range(5..window_size.0 - 5),
            y: rng.random_range(5..(window_size.1 - (window_size.1 / 4)))
        };

        let _ = renderer.paint_tile(tip_mat, tip_pnt.clone());
        all_tips.push(tip_pnt);
    };
    //while let Some(top) = all_tips.pop() {
    //    println!("{}, {}", top.x, top.y);
    //}
    Ok(())
}
