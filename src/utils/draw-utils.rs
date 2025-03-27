use std::io::{self, Write, Stdout, Result};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal::{self, size, WindowSize},
    cursor, 
    style::{self, Stylize, StyledContent, Color},
};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Point {
    x: u16,
    y: u16
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

struct Shifter {
    gap: u16,
    x_shift: u16,
    y_shift: u16
}

#[derive(Serialize, Deserialize)]
struct Rect {
    s_pnt: Point,
    e_pnt: Point
}

const MIN_WINDOW_WIDTH: u16 = 32;
const MIN_WINDOW_LENGTH: u16 = 32;
const LIP_SIZE: u16 = 3;


fn paint_outline(trk: &mut Stdout, token: StyledContent<char>, pnt: &mut Point, shift: &mut Shifter) -> Result<()> {
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

fn paint_tile(trk: &mut Stdout, token: StyledContent<char>, pnt: Point) -> Result<()> {
    trk.queue(cursor::MoveTo(pnt.x, pnt.y))?.queue(style::PrintStyledContent(token))?;
    Ok(())
}


fn paint_rect(trk: &mut Stdout, token: StyledContent<char>, rect: Rect) -> Result<()> {
    for y in rect.s_pnt.y..rect.e_pnt.y {
        for x in rect.s_pnt.x..rect.e_pnt.x {
            paint_tile(trk, token, Point{x,y})?;
        }
    }
    Ok(())
}

