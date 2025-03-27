mod pot;
mod utils;

use std::io::{self, Write};
use crossterm::{ExecutableCommand, terminal};
use crate::pot::draw_pot;

fn main() -> io::Result<()> {
    let mut trk = io::stdout();
    trk.execute(terminal::Clear(terminal::ClearType::All))?;
    
    if let Err(e) = draw_pot(&mut trk) {
        println!("{:?}", e)
    }
    
    trk.flush()?;   
    Ok(())
}
