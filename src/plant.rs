use std::io::{Stdout, Result};
use crossterm::{
    QueueableCommand,
    cursor, 
    style::{self, StyledContent},
};

use crate::utils::draw_utils::{Point, Rect, Shifter, paint_rect, paint_outline, MIN_WINDOW_WIDTH, MIN_WINDOW_LENGTH, LIP_SIZE};

pub fn plant(seed: u32) -> Result<()>{
    
    Ok(()) 
}
