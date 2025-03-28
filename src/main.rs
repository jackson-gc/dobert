mod pot;
mod utils;
mod plant;

use rand::Rng;
use std::io::{self, Write};
use crossterm::{ExecutableCommand, terminal};
use serde::{Serialize, Deserialize};

use crate::pot::draw_pot;
use crate::plant::plant;

#[derive(Serialize, Deserialize)]
pub struct PlantInfo {
    seed: u32, 
}

fn main() -> io::Result<()> {
    let mut trk = io::stdout();
    trk.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut plant_info = PlantInfo {
        seed: 0
    };
    
    if let Err(e) = draw_pot(&mut trk) {
        println!("{:?}", e);
    }

    // 8-digit rng
    let seed: u32 = rand::thread_rng().gen_range(10_000_000..100_000_000);
    plant_info.seed = seed;


    println!("{}", seed);
    
    

    
    trk.flush()?;   
    Ok(())
}
