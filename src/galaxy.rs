use {
    crate::object::Object,
    macroquad::prelude::*,
    std::{f32::consts::PI, iter},
};

const MIN_RADIUS: f32 = 30.;
const SLEEVES: usize = 5;

pub struct SpiralGalaxy {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub objects_count: usize,
}

pub fn spiral_galaxy(galaxy: SpiralGalaxy) -> Vec<Object> {
    let curvature = (5. * PI / 4.).powf(-1.) * (galaxy.radius / MIN_RADIUS).ln();

    (0..galaxy.objects_count)
        .map(|_| {
            let radius = rand::gen_range(MIN_RADIUS, galaxy.radius);
            let sleeve = rand::gen_range(0, SLEEVES);

            let angle = curvature.powf(-1.) * (radius / MIN_RADIUS).ln()
                + sleeve as f32 * (2. * PI) / SLEEVES as f32
                + PI / rand::gen_range(3f32, 4.);

            let mass = rand::gen_range(0.01, 1f32).powi(10) * 1.;

            let u = Vec2::from_angle(angle);
            let pos = galaxy.pos + -u * radius;

            let vel =
                galaxy.vel + u.rotate(Vec2::new(0., 1.)) * ((galaxy.mass + mass) / radius).sqrt();

            Object {
                pos,
                vel,
                mass,
                radius: 0.5,
                ..Default::default()
            }
        })
        .chain(iter::once(Object {
            mass: galaxy.mass,
            radius: 5.,
            pos: galaxy.pos,
            vel: galaxy.vel,
            ..Default::default()
        }))
        .collect()
}
