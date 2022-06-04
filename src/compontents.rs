use bevy::{prelude::Component, math::Vec2};

// region:      Common Components 

#[derive(Component, Debug, PartialEq)]
pub struct ID {
    pub id: u32,
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn normalize(&self) -> Vec2 {
        return Vec2::new(self.x, self.y).normalize();
    }

    pub fn length_squared(&self) -> f32 {
        return Vec2::new(self.x, self.y).length_squared();
    }
}

// endregion:   Common Components 