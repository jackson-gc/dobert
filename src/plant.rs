use std::io::Result;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crossterm::style::Stylize;

use crate::utils::draw_utils::{Renderer, Point, Token};

pub fn plant(renderer: &mut Renderer, seed: u64) -> Result<()> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    draw_branch(renderer, &mut rng, 16)?;
    
    
    Ok(()) 
}



fn draw_branch(renderer: &mut Renderer, rng: &mut ChaCha8Rng, stems: u16) -> Result<()> {
    let window_size = renderer.get_window_ctx();
    let tip: Token = '%'.dark_green();
    
    for _ in 0..stems{
        let _ = renderer.paint_tile(tip, Point {
            x: rng.random_range(5..window_size.0 - 5),
            y: rng.random_range(5..(window_size.1 - (window_size.1 / 4)))
        });
    };

    Ok(())
}
