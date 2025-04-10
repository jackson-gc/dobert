use std::io::Result;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use crossterm::style::Stylize;

use crate::utils::draw::{Renderer, Point, Token};

const STEM_COUNT:usize = 12;

pub struct Plant {
    nut: Point,
    rng: ChaCha8Rng,
    branches: Vec<Branch>
}

impl Plant {
    pub fn new(rng: ChaCha8Rng) -> Self {
       Plant {
            nut: Point::new();,
            branches: Vec::new();,
            rng,
        }
    }
}

struct Branch {
    nut: Point,
    tip: Point
}


pub fn plant(renderer: &mut Renderer, rng_seed: u64, nut: &mut Point) -> Plant {
    let mut rng = ChaCha8Rng::seed_from_u64(rng_seed);
    let plant = Plant::new();
    let window_size = renderer.window_size;

    nut.x = window_size.0 / 2;
    nut.y = window_size.1 - (window_size.1 / 8) - 1;
    match renderer.draw_tile('#'.red(), nut){
        _ => {}
    }
    let _all_tips: Vec<Point> = find_tips(renderer, &mut rng);

    plant
}

fn draw_branches(renderer: &mut Renderer, rng: &mut ChaCha8Rng){

}


fn find_tips(renderer: &mut Renderer, rng: &mut ChaCha8Rng) -> Vec::<Point> {
    let tip_mat: Token = '%'.dark_green();
    let ws = renderer.window_size;
    let mut all_tips = Vec::<Point>::new();
    let max_x = ws.0 - 5;
    let max_y = ws.1 - (ws.1 / 6);
    for _ in 0..STEM_COUNT {
        let tip_pnt = Point {
            x: rng.random_range(5..max_x),
            y: rng.random_range(5..max_y)
        };
        let _ = renderer.draw_tile(tip_mat, &tip_pnt);
        all_tips.push(tip_pnt);
    };
    all_tips
}
