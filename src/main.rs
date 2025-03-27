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


fn main() -> io::Result<()> {
    let mut trk = io::stdout();
    trk.execute(terminal::Clear(terminal::ClearType::All))?;

    if let Err(e) = draw_pot(&mut trk) {println!("{:?}", e)}
    let _ = trk.flush();

    Ok(())
}



fn draw_pot(trk: &mut Stdout) -> Result<()> {
    let window_width: u16 = size()?.0;
    let window_length: u16 = size()?.1;

    if window_width < MIN_WINDOW_WIDTH {
        panic!("Window width is too small (must be >{})", MIN_WINDOW_WIDTH);
    }

    if window_length < MIN_WINDOW_LENGTH {
        panic!("Winow length is too small (must be >{})", MIN_WINDOW_LENGTH);
    }

    let start_index: u16 = window_width - MIN_WINDOW_WIDTH;
    let margin: u16 = start_index / 4;
    let size: u16 = window_width - margin * 2;
    let pot_depth: u16 = window_length / 8;
    let depth: u16 = window_length - pot_depth;

    let clr: Color = Color::Rgb{r: 160, g: 130, b: 90};
    let pot_hard_mat: StyledContent<char> = '█'.with(clr);

    paint_rect(trk, pot_hard_mat, Rect {
        s_pnt: Point {
            x: margin,
            y: depth
        },
        e_pnt: Point {
            x: margin + size,
            y: depth + 1
        }
    })?;

    let mut draw_pnt = Point {
        x: margin + LIP_SIZE,
        y: depth
    };

    let mut draw_shift = Shifter {
        gap: size - ((margin / 4) + LIP_SIZE * 2), 
        x_shift: 1, 
        y_shift: 1
    };
    
    for i in 1..pot_depth {
        if i & 2 == 0 {
            draw_shift.x_shift = 0;
        } else {
            draw_shift.x_shift = 1;
        }

        if i == pot_depth - pot_depth / 4 {
            let end_width = draw_pnt.x + draw_shift.gap;

            paint_rect(trk, pot_hard_mat, Rect {
                s_pnt: draw_pnt.clone(),
                e_pnt: Point {
                    x: end_width,
                    y: draw_pnt.y + 1
                }
            })?;
        }
        paint_outline(trk, pot_hard_mat, &mut draw_pnt, &mut draw_shift)?;
    }

    let end_width: u16 = draw_pnt.x + draw_shift.gap + 1;
    let end_length: u16 = draw_pnt.y;
    paint_rect(trk, pot_hard_mat, Rect {
        s_pnt: draw_pnt,
        e_pnt: Point {
            x: end_width,
            y: end_length
        }
    })?;

    Ok(())

}


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

