use std::io::{self, Write, Stdout, Result};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize, StyledContent}
};

struct Point {
    x: u16,
    y: u16
}

struct Rect {
    s_pnt: Point,
    e_pnt: Point
}

fn main() -> io::Result<()> {
    let mut trk = io::stdout();
    trk.execute(terminal::Clear(terminal::ClearType::All))?;


    let draw = Rect {
        s_pnt: Point {
            x: 8,
            y: 4
        },
        e_pnt: Point {
            x: 48,
            y: 24
        }
    };


    match paint_rect(&mut trk, '█'.yellow(), draw) {
        Err(e) => {println!("{:?}", e)},
        _ => {}
    }
    trk.flush()?;
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

