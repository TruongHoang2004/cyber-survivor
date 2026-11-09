mod ally;
mod character;
mod core;
mod enemy;
mod skill;
mod ui;
mod weapon;

use crate::core::world::GameWorld;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "CYBER SURVIVOR (Grand-Scale Modular Rust Action)".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = GameWorld::new().await;

    loop {
        let dt = get_frame_time().min(0.05);
        world.step(dt);
        next_frame().await;
    }
}
