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

const MIN_WINDOW_SIZE: u16 = 32;


fn main() -> io::Result<()> {
    let mut trk = io::stdout();
    trk.execute(terminal::Clear(terminal::ClearType::All))?;

    if let Err(e) = draw_pot(&mut trk) {println!("{:?}", e)}
    let _ = trk.flush();

    Ok(())
}



fn draw_pot(trk: &mut Stdout) -> Result<()> {
    let window_size: u16 = size()?.0;
    
    if window_size < MIN_WINDOW_SIZE {
        panic!("Window size is too small (must be >32)")
    }

    let start_index: u16 = window_size - MIN_WINDOW_SIZE;
    let margin: u16 = start_index / 4;
    let size: u16 = window_size - (margin * 2);

    println!("start_index: {:?}, size: {:?}, margin: {:?}, total: {:?}",start_index, size, margin, window_size);

    let clr: Color = Color::Rgb{r: 160, g: 130, b: 90};
    let pot_hard_mat: StyledContent<char> = '█'.with(clr);

    paint_rect(trk, pot_hard_mat, Rect {
        s_pnt: Point {
            x: margin,
            y: 20
        },
        e_pnt: Point {
            x: margin + size,
            y: 20 + 1
        }
    })?;



    let mut draw_pnt = Point {
        x: margin + (margin / 4),
        y: 21
    };

    let mut draw_shift = Shifter {
        gap: size - (margin / 2), 
        x_shift: 1, 
        y_shift: 1
    };
    
    for _i in 1..12 {
        paint_outline(trk, pot_hard_mat, &mut draw_pnt, &mut draw_shift)?;
    }




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

