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
use crate::utils::draw::Renderer;

#[derive(Serialize, Deserialize, Default)]
pub struct PlantInfo {
    seed: u64,
}

impl PlantInfo {
    fn new() -> Self {
        Default::default()
    }
}

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new();
    let mut plant_info = PlantInfo::new();
    renderer.stdout_mut().execute(terminal::Clear(terminal::ClearType::All))?;
    
    draw_pot(&mut renderer)?;
    if plant_info.seed == 0 {
        plant_info.seed = rand::rng().random_range(10_000_000..100_000_000);
    }
    
    let mut nut = utils::draw::Point::new();
    plant(&mut renderer, plant_info.seed, &mut nut)?;

    renderer.flush()?;
    Ok(())
}

