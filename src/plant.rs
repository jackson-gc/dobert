use std::io::Result;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crossterm::style::Stylize;

use crate::utils::draw_utils::{Renderer, Point, Token};

pub fn plant(renderer: &mut Renderer, seed: u64) -> Result<()> {
    let window_size = renderer.get_window_ctx();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    
    // Fix deprecated method - use random_range instead of gen_range
    let point: u16 = rng.random_range(1..window_size.0);
    
    println!("{}/{}", point, window_size.0);



    let cursor: Token = '#'.red();
    renderer.paint_tile(cursor, Point {
        x: point,
        y: 20
    })?;

    Ok(()) 
}
