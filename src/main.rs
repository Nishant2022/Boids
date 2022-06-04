//#![allow(dead_code)]

use bevy::prelude::*;
use boid::BoidPlugin;

mod boid;
mod compontents;

// region:      Asset Constants

    const BOID_SPRITE: &str = "arrow.png";

    const SPRITE_SCALE: f32 = 0.1;

// endregion:   Asset Constants

// region:      Resources

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
}

pub struct BoidSettings {
    pub boid_view_distance_sqrd: f32,
    pub boid_max_speed: f32,
}

// endregion:   Resources

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: String::from("Nishant's Boids"),
            width: 600.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BoidPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    
    // Add camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Add WinSize resource
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WinSize {w: win_w, h: win_h};
    commands.insert_resource(win_size);

    // Add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(BOID_SPRITE)
    };
    commands.insert_resource(game_textures);

    // Add BoidSettings resource
    let boid_settings = BoidSettings {
        boid_view_distance_sqrd: 10000.0,
        boid_max_speed: 3.0
    };
    commands.insert_resource(boid_settings);

}