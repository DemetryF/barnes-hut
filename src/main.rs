mod galaxy;
mod object;
mod quadtree;
mod state;

use {
    crate::{
        galaxy::{SpiralGalaxy, spiral_galaxy},
        quadtree::TreeParams,
        state::State,
    },
    macroquad::prelude::*,
    std::time::Instant,
};

const SPEED: f32 = 40.;
const THETA: f32 = 0.7;

#[macroquad::main("Barnes-Hut")]
async fn main() {
    let params = TreeParams {
        max_depth: 9,
        leaf_size: Vec2::new(12., 12.),
        center: Vec2::ZERO,
    };

    let mut galaxy1 = spiral_galaxy(SpiralGalaxy {
        pos: Vec2::new(0., 0.),
        vel: Vec2::new(0., 0.),
        mass: 2000.,
        radius: 190.,
        objects_count: 1000,
    });
    let mut galaxy2 = spiral_galaxy(SpiralGalaxy {
        pos: Vec2::new(300., 0.),
        vel: Vec2::new(0., -(2500f32 / 300.).sqrt()),
        mass: 500.,
        radius: 90.,
        objects_count: 300,
    });

    let mut objects = Vec::new();
    objects.append(&mut galaxy1);
    objects.append(&mut galaxy2);

    let mut state = State::new(objects, params);

    let mut delta_time = 0.;

    loop {
        let frame_start = Instant::now();

        clear_background(BLACK);

        for obj in &state.objects {
            draw_circle(
                screen_width() / 2. + obj.pos.x,
                screen_height() / 2. + obj.pos.y,
                obj.radius,
                Color::from_rgba(255, 255, 255, 255),
            );
        }

        state.update(SPEED * delta_time, THETA);

        next_frame().await;

        delta_time = frame_start.elapsed().as_secs_f32();
    }
}
