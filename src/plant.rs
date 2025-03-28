use std::io::{Stdout, Result};
use crossterm::{
    QueueableCommand,
    cursor, 
    style::{self, StyledContent},
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;
use crate::utils::draw_utils::{Point, Rect, Shifter, paint_rect, paint_outline, MIN_WINDOW_WIDTH, MIN_WINDOW_LENGTH, LIP_SIZE};

pub fn plant(seed: u64) -> Result<()>{
    let mut rng = ChaCha12Rng::seed_from_u64(seed);
    
    Ok(()) 
}
