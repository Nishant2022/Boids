use bevy::prelude::*;
use rand::Rng;

use crate::{SPRITE_SCALE, GameTextures, WinSize, BoidSettings, boid_logic::Boid};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, boid_spawn_system)
            .add_system(boid_update_system)
            .add_system(boid_update_group_system.after(boid_update_system));
        
        #[cfg(target_arch = "wasm32")]
        {
            app.add_plugin(bevy_web_resizer::Plugin);
        }
    
    }
}

fn boid_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let mut boid_vec: BoidGroup = BoidGroup{ boids: Vec::new() };
    for i in 0..100 {
        
        // Get position
    
        let x_pos: f32 = rand::thread_rng().gen_range((-win_size.w/2.0)..(win_size.w/2.0));
        let y_pos: f32 = rand::thread_rng().gen_range((-win_size.h/2.0)..(win_size.h/2.0));
        let dx:f32 = rand::thread_rng().gen_range(-3.0..3.0);
        let dy:f32 = rand::thread_rng().gen_range(-3.0..3.0);
        
        let boid_component: Boid = Boid {
            x: x_pos,
            y: y_pos,
            dx: dx,
            dy: dy,
            id: i,
        };
        let boid: Boid = boid_component.clone();

        // Add player
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform {
                    translation: Vec3::new(x_pos, y_pos, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE/1.2, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(boid_component);
        
        boid_vec.boids.push(boid);
    }

    commands.insert_resource(boid_vec);
}

fn boid_update_system(mut query: Query<(&mut Boid, &mut Transform)>, boids: Res<BoidGroup>, settings: Res<BoidSettings>, win_size: Res<WinSize>) {
    // Loops through all boid entities and updates positions based on BoidGroup
    
    for (mut boid, mut transform) in query.iter_mut() {
        
        // updates boid
        boid.update(&boids.boids, &settings);

        // If the margin is greater than zero, boids bounce off walls
        // otherwise they teleport to other side of screen
        if settings.margin > 0.0 {
            if boid.x < -win_size.w / 2.0 + settings.margin {boid.dx += 0.5}
            else if boid.x > win_size.w / 2.0 - settings.margin {boid.dx -= 0.5}
            if boid.y < -win_size.h / 2.0 + settings.margin {boid.dy += 0.5}
            else if boid.y > win_size.h / 2.0 - settings.margin {boid.dy -= 0.5}
        } else {
            if boid.x < -win_size.w / 2.0 {boid.x += win_size.w}
            else if boid.x > win_size.w / 2.0 {boid.x -= win_size.w}
            if boid.y < -win_size.h / 2.0 {boid.y += win_size.h}
            else if boid.y > win_size.h / 2.0 {boid.y -= win_size.h}
        }

        // Set the translation of boid entities
        transform.translation.x = boid.x;
        transform.translation.y = boid.y;

        // Set the rotation of boid entities
        transform.rotation = Quat::from_rotation_z(-Vec2::new(boid.dx, boid.dy).angle_between(Vec2::X));

    }
}

fn boid_update_group_system(query: Query<&Boid>, mut boids: ResMut<BoidGroup>) {
    // Used to update boid group
    boids.boids = Vec::new();
    for boid in query.iter() {
        boids.boids.push(boid.clone());
    }
}


struct BoidGroup {
    // Struct use to hold a copy of all boid
    // entities in order to properly update positions
    boids: Vec<Boid>,
}

