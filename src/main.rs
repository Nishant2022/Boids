use bevy::{prelude::*, ecs::event::Events, window::WindowResized};
use boid_plugin::BoidPlugin;
use text::TextPlugin;

mod boid_plugin;
mod boid_logic;
mod compontents;
mod text;

// region:      Asset Constants

    const BOID_SPRITE: &str = "images/arrow.png";

    const SPRITE_SCALE: f32 = 0.075;

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
    pub margin: f32,
    pub boid_view_distance_sqrd: f32,
    pub boid_max_speed: f32,
    pub cohesion_multiplier: f32,
    pub min_distance: f32,
    pub separation_multiplier: f32,
    pub alignment_multiplier: f32,
}

// endregion:   Resources

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: String::from("Nishant's Boids"),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BoidPlugin)
        .add_startup_system(setup_system)
        .add_plugin(TextPlugin)
        .add_system(window_resize_system)
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
        margin: 50.0,
        boid_view_distance_sqrd: 250.0,
        boid_max_speed: 2.5,
        cohesion_multiplier: 0.007,
        min_distance: 10.0,
        separation_multiplier: 0.05,
        alignment_multiplier: 0.1,
    };
    commands.insert_resource(boid_settings);

}

fn window_resize_system(resize_event: Res<Events<WindowResized>>, mut win_size: ResMut<WinSize>) {
    
    // Checks for resize event and updates win_size resource
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        win_size.w = e.width;
        win_size.h = e.height;
    }
}