mod pot;
mod utils;
mod plant;

use rand::Rng;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use serde::{Serialize, Deserialize};
use std::io;

use crate::pot::draw_pot;
use crate::plant::plant;
use crate::utils::draw_utils::Renderer;

#[derive(Serialize, Deserialize)]
pub struct PlantInfo {
    seed: u64, 
}

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new();
    renderer.stdout_mut().execute(terminal::Clear(terminal::ClearType::All))?;

    let mut plant_info = PlantInfo {
        seed: 0
    };
    
    if let Err(e) = draw_pot(&mut renderer) {
        println!("{:?}", e);
    }

    // 8-digit rng - using random_range instead of gen_range
    let seed: u64 = rand::rng().random_range(10_000_000..100_000_000);
    plant_info.seed = seed;
    
    if let Err(e) = plant(&mut renderer, plant_info.seed) {
        println!("{:?}", e);
    }

    renderer.flush()?;
    Ok(())
}

