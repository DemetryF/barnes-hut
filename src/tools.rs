use {
    crate::{galaxy::SpiralGalaxy, object::Object},
    macroquad::prelude::*,
    std::f32::consts::PI,
};

pub fn make_orbit_each_other(a: &mut dyn Kinematic, b: &mut dyn Kinematic) {
    let dist = a.pos() - b.pos();

    let v = Vec2::from_angle(dist.to_angle() + PI / 2.).normalize();

    *a.vel_mut() = v * f32::sqrt(b.mass() * b.mass() / (dist.length() * (a.mass() + b.mass())));

    *b.vel_mut() = -v * f32::sqrt(a.mass() * a.mass() / (dist.length() * (a.mass() + b.mass())));
}

pub trait Kinematic {
    fn pos(&self) -> Vec2;
    fn vel_mut(&mut self) -> &mut Vec2;
    fn mass(&self) -> f32;
}

impl Kinematic for Object {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn vel_mut(&mut self) -> &mut Vec2 {
        &mut self.vel
    }

    fn mass(&self) -> f32 {
        self.mass
    }
}

impl Kinematic for SpiralGalaxy {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn vel_mut(&mut self) -> &mut Vec2 {
        &mut self.vel
    }

    fn mass(&self) -> f32 {
        self.mass
    }
}
