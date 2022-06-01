use bevy::prelude::*;

// region:      Asset Constants

    const BOID_SPRITE: &str = "arrow.png";
    const PLAYER_SIZE: (f32, f32) = (280., 280.);

// endregion:   Asset Constants

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
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(BOID_SPRITE),
        ..Default::default()
    });
}