mod plant;
mod pot;
mod utils;

use crossterm::{terminal, ExecutableCommand};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::io::Result;

use crate::plant::{pot_the_plant, Plant};
use crate::pot::draw_pot;
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

fn main() -> Result<()> {
    let mut renderer = Renderer::new();
    let mut plant_info = PlantInfo::new();
    renderer
        .stdout_mut()
        .execute(terminal::Clear(terminal::ClearType::All))?;

    plant_info.seed = 58008;

    draw_pot(&mut renderer)?;
    if plant_info.seed == 0 {
        plant_info.seed = rand::rng().random_range(10_000_000..100_000_000);
    }

    let mut plant = Plant::new(ChaCha8Rng::seed_from_u64(plant_info.seed));
    pot_the_plant(&mut renderer, &mut plant)?;

    renderer.clean()?;
    Ok(())
}
