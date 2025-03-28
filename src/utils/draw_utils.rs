pub use std::io::{Stdout, Result};
pub use crossterm::{
    QueueableCommand,
    cursor, 
    terminal::size,
    style::{self, StyledContent},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16
}

#[derive(Clone)]
pub struct Shifter {
    pub gap: u16,
    pub x_shift: u16,
    pub y_shift: u16
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Rect {
    pub s_pnt: Point,
    pub e_pnt: Point
}

pub const MIN_WINDOW_WIDTH: u16 = 32;
pub const MIN_WINDOW_LENGTH: u16 = 32;
pub const LIP_SIZE: u16 = 3;

pub fn get_window_ctx() -> (u16, u16) {
    match size() {
        Ok((w,l)) => (w,l),
        _ => (0,0)
    }
}


pub fn paint_outline(trk: &mut Stdout, token: StyledContent<char>, pnt: &mut Point, shift: &mut Shifter) -> Result<()> {
    paint_tile(trk, token, Point {
        x: pnt.x,
        y: pnt.y
    })?;
    
    paint_tile(trk, token, Point {
        x: pnt.x + shift.gap,
        y: pnt.y
    })?;
    
    pnt.x += shift.x_shift;
    pnt.y += shift.y_shift;
    shift.gap -= shift.x_shift * 2;

    Ok(())
}

pub fn paint_tile(trk: &mut Stdout, token: StyledContent<char>, pnt: Point) -> Result<()> {
    trk.queue(cursor::MoveTo(pnt.x, pnt.y))?.queue(style::PrintStyledContent(token))?;
    Ok(())
}


pub fn paint_rect(trk: &mut Stdout, token: StyledContent<char>, rect: Rect) -> Result<()> {
    for y in rect.s_pnt.y..rect.e_pnt.y {
        for x in rect.s_pnt.x..rect.e_pnt.x {
            paint_tile(trk, token, Point{x,y})?;
        }
    }
    Ok(())
}
