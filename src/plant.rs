use std::io::{Result, Stdout};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::utils::draw_utils::{get_window_ctx, paint_tile};

pub fn plant(trk: &mut Stdout, seed: u64) -> Result<()>{
    let window_size = get_window_ctx();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    
    let point: u16 = rng.random_range(1..window_size.0);
    
    println!("{}/{}", point, window_size.0);


    Ok(()) 
}
