use {
    crate::object::Object,
    macroquad::prelude::*,
    std::{f32::consts::PI, iter},
};

pub struct SpiralGalaxy {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub min_radius: f32,
    pub max_radius: f32,
    pub period: f32,
    pub sleeves: usize,
    pub curvature_angle: f32,
    pub objects_count: usize,
}

pub fn spiral_galaxy(galaxy: SpiralGalaxy) -> Vec<Object> {
    let curvature = galaxy.curvature_angle.powf(-1.) * (galaxy.max_radius / galaxy.min_radius).ln();

    (0..galaxy.objects_count)
        .map(|_| {
            let radius = rand::gen_range(
                galaxy.min_radius * galaxy.max_radius,
                galaxy.max_radius.powi(2),
            ) / galaxy.max_radius;

            let sleeve = rand::gen_range(0, galaxy.sleeves);

            let angle = curvature.powf(-1.) * (radius / galaxy.min_radius).ln()
                + sleeve as f32 * (2. * PI) / galaxy.sleeves as f32
                + PI / rand::gen_range(3f32, 4.);

            let mass = rand::gen_range(0.01, 1f32).powf(10.);

            let u = Vec2::from_angle(angle);
            let pos = galaxy.pos + -u * radius;

            let speed = {
                (galaxy.mass
                    * (2.0 / radius - ((2.0 * PI / galaxy.period).powi(1) / galaxy.mass).cbrt()))
                .sqrt()
            };

            let local_vel =
                -u.rotate(Vec2::new(0., 1.)) * speed * (1. + rand::gen_range(-0.005, 0.005));

            let vel = galaxy.vel + local_vel;

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
