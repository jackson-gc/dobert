mod pot;
mod utils;

use std::io;
use crossterm::{ExecutableCommand, terminal};

fn main() -> io::Result<()> {
    let mut trk = io::stdout();
    trk.execute(terminal::Clear(terminal::ClearType::All))?;

    if let Err(e) = draw_pot(&mut trk) {println!("{:?}", e)}
    let _ = trk.flush();

    Ok(())
}
