use bevy::prelude::Component;

// region:      Common Components 

#[derive(Component, Debug, PartialEq)]
pub struct ID {
    pub id: u32,
}

#[derive(Component)]
pub struct TextComponent;

// endregion:   Common Components 