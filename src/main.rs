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
    macroquad::{miniquad::window::set_window_size, prelude::*},
    std::{f32::consts::PI, time::Instant},
};

const SPEED: f32 = 5.;
const THETA: f32 = 1.2;

#[macroquad::main("Barnes-Hut")]
async fn main() {
    set_window_size(800, 800);

    let params = TreeParams {
        max_depth: 6,
        leaf_size: Vec2::new(25., 25.),
        center: Vec2::ZERO,
    };

    let mut galaxy1 = spiral_galaxy(SpiralGalaxy {
        pos: Vec2::new(0., 0.),
        vel: Vec2::new(0., 0.),
        mass: 500000.,
        max_radius: 200.,
        objects_count: 10000,
        min_radius: 20.,
        period: 20.,
        sleeves: 4,
        curvature_angle: 3. * PI / 4.,
    });

    let mut objects = Vec::new();
    objects.append(&mut galaxy1);

    let mut state = State::new(objects, params);

    let mut delta_time: f32 = 0.;

    loop {
        let frame_start = Instant::now();

        clear_background(BLACK);

        state.update(SPEED * delta_time.min(0.02f32), THETA);

        for obj in &state.objects {
            draw_circle(
                screen_width() / 2. + obj.pos.x,
                screen_height() / 2. + obj.pos.y,
                obj.radius,
                Color::from_rgba(255, 255, 255, 255),
            );
        }

        next_frame().await;

        delta_time = frame_start.elapsed().as_secs_f32();
    }
}
