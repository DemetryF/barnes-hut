use {
    crate::{
        galaxy::{SpiralGalaxy, spiral_galaxy},
        quadtree::TreeParams,
        state::State,
    },
    macroquad::prelude::*,
    std::{f32::consts::PI, time::Instant},
};

#[test]
pub fn bench_galaxy() {
    let galaxy = SpiralGalaxy {
        mass: 500_000.,
        min_radius: 100.,
        max_radius: 350.,
        sleeves: 4,
        curvature_angle: 5. * PI / 4.,
        objects_count: 20_000,
        ..Default::default()
    };

    let params = TreeParams {
        max_depth: 9,
        leaf_size: Vec2::new(8., 8.),
        center: Vec2::ZERO,
    };

    let theta = 1.;
    let delta_time = 0.1;

    let mut state = State::new(spiral_galaxy(galaxy), params);

    state.init(theta);

    let start = Instant::now();

    for _ in 0..500 {
        state.update(delta_time, theta);
    }

    let secs_f32 = start.elapsed().as_secs_f32();

    println!("500 steps: {}", secs_f32);
}
