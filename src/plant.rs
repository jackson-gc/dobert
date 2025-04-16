use crossterm::style::Stylize;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use std::io::Result;

use crate::utils::draw::{Point, Renderer, Token};

const STEM_COUNT: usize = 86;

pub struct Plant {
    nut: Point,
    rng: ChaCha8Rng,
    branches: Vec<Branch>,
}

impl Plant {
    pub fn new(rng: ChaCha8Rng) -> Self {
        Plant {
            nut: Point::new(),
            branches: Vec::new(),
            rng,
        }
    }
}

struct Branch {
    tip: Point,
    body: Vec<Point>,
}

pub fn pot_the_plant(renderer: &mut Renderer, plant: &mut Plant) -> Result<()> {
    let window_size = renderer.window_size;
    plant.nut.x = window_size.0 / 2;
    plant.nut.y = window_size.1 - (window_size.1 / 8) - 1;
    renderer.draw_tile('#'.red(), &plant.nut)?;

    plant.branches = gen_branches(renderer, &mut plant.rng);

    draw_branches(renderer, plant)?;
    Ok(())
}

pub fn draw_branches(renderer: &mut Renderer, plant: &Plant) -> Result<()> {
    for branch in &plant.branches {
        let x0 = plant.nut.x as i32;
        let y0 = plant.nut.y as i32;
        let x1 = branch.tip.x as i32;
        let y1 = branch.tip.y as i32;

        let steep = (y1 - y0).abs() > (x1 - x0).abs();

        let (mut x0, mut y0, mut x1, mut y1) = if steep {
            (y0, x0, y1, x1)
        } else {
            (x0, y0, x1, y1)
        };

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = (y1 - y0).abs();
        let mut err = dx / 2;
        let mut y = y0;
        let y_step = if y0 < y1 { 1 } else { -1 };

        for x in x0..=x1 {
            let tile = if steep {
                Point {
                    x: y as u16,
                    y: x as u16,
                }
            } else {
                Point {
                    x: x as u16,
                    y: y as u16,
                }
            };

            renderer.draw_tile('@'.green(), &tile)?;

            err -= dy;
            if err < 0 {
                y += y_step;
                err += dx;
            }
        }
    }
    Ok(())
}

fn gen_branches(renderer: &mut Renderer, rng: &mut ChaCha8Rng) -> Vec<Branch> {
    let tip_mat: Token = '%'.dark_green();
    let ws = renderer.window_size;
    let mut branches = Vec::<Branch>::new();
    let max_x = ws.0 - 5;
    let max_y = ws.1 - (ws.1 / 6);

    for _ in 0..STEM_COUNT {
        let tip_pnt = Point {
            x: rng.random_range(5..max_x),
            y: rng.random_range(5..max_y),
        };
        let _ = renderer.draw_tile(tip_mat, &tip_pnt);
        branches.push(Branch {
            tip: tip_pnt,
            body: Vec::new(),
        });
    }
    branches
}
