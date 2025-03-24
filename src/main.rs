use std::io::{self, Write, Stdout, Result};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize, StyledContent}
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
struct Point {
   x: u16,
    y: u16
}


#[derive(Serialize, Deserialize)]
struct Rect {
    s_pnt: Point,
    e_pnt: Point
}




fn main() -> io::Result<()> {
    

    let point01 = Point{x: 12, y: 12};
    let point_ser = to_string(&point01);
    
    if point_ser.is_ok() {
        println!("{}", point_ser.ok().unwrap());
    } else {
        println!("{:#?}", point_ser.err());
    }


    return Ok(());

    let mut trk = io::stdout();
    trk.execute(terminal::Clear(terminal::ClearType::All))?;

    match draw_pot(&mut trk, 32, 40, 20) {
        Err(e) => {println!("{:?}", e)},
        _ => {}
    };
    let _ = trk.flush();

    Ok(())
}



fn draw_pot(trk: &mut Stdout, size: u16, margin: u16, mut elevation: u16) -> Result<()> {
    let potMat: StyledContent<char> = '='.red();
    let mut shift: u16 = 2;
    let draw = Rect {
        s_pnt: Point {
            x: margin,
            y: elevation
        },
        e_pnt: Point {
            x: margin + size + shift,
            y: elevation + 1
        }
    };

    paint_rect(trk, potMat, draw)?;
    elevation += 1;
    shift += 1;

    paint_tile(trk, potMat, Point{x:margin + shift, y:elevation})?;
    paint_tile(trk, potMat, Point{x:(margin + size - shift + 1), y:elevation})?;
    elevation += 1;

    paint_tile(trk, potMat, Point{x:margin + shift, y:elevation})?;
    paint_tile(trk, potMat, Point{x:(margin + size - shift + 1), y:elevation})?;
    shift += 1;

    paint_tile(trk, potMat, Point{x:margin + shift, y:elevation})?;
    paint_tile(trk, potMat, Point{x:(margin + size - shift + 1), y:elevation})?;
    elevation += 1;
    shift += 1;

    paint_tile(trk, potMat, Point{x:margin + shift, y:elevation})?;
    paint_tile(trk, potMat, Point{x:(margin + size - shift + 1), y:elevation})?;
    elevation += 1;
    shift += 1;

    paint_tile(trk, potMat, Point{x:margin + shift, y:elevation})?;
    paint_tile(trk, potMat, Point{x:(margin + size - shift + 1), y:elevation})?;
    shift += 1;

    paint_tile(trk, potMat, Point{x:margin + shift, y:elevation})?;
    paint_tile(trk, potMat, Point{x:(margin + size - shift + 1), y:elevation})?;
    elevation += 1;
    shift += 1;

    paint_tile(trk, potMat, Point{x:margin + shift, y:elevation})?;
    paint_tile(trk, potMat, Point{x:(margin + size - shift + 1), y:elevation})?;
    shift += 1;

    paint_tile(trk, potMat, Point{x:margin + shift, y:elevation})?;
    paint_tile(trk, potMat, Point{x:(margin + size - shift + 1), y:elevation})?;


    elevation -= 1;
    shift += 1;
    
    paint_rect(trk, potMat, Rect {
        s_pnt: Point {
            x: margin + shift,
            y: elevation
        },
        e_pnt: Point {
            x: margin + size - shift + 2,
            y: elevation + 1
        }
    })?;


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

