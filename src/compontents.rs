use bevy::prelude::Component;

// region:      Common Components 

#[derive(Component, Debug)]
pub struct ID {
    pub id: u32,
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// endregion:   Common Components 