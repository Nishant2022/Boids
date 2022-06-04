use bevy::prelude::*;
use rand::Rng;

use crate::{SPRITE_SCALE, GameTextures, WinSize, compontents::{Velocity, ID}, BoidSettings};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum BoidSystemLabels {
    UpdatingVelocity,
}
pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, boid_spawn_system)
            .add_system(boid_movement_system.after(BoidSystemLabels::UpdatingVelocity))
            .add_system(boid_cohesion_system.label(BoidSystemLabels::UpdatingVelocity));
    }
}

fn boid_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {

    for i in 0..100 {
        
            // Get position
        
            let x_pos = rand::thread_rng().gen_range((-win_size.w/2.0)..(win_size.w/2.0));
            let y_pos = rand::thread_rng().gen_range((-win_size.h/2.0)..(win_size.h/2.0));
            
            // Add player
            commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.player.clone(),
                    transform: Transform {
                        translation: Vec3::new(x_pos, y_pos, 10.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })

                // Add velocity
                .insert(Velocity {x: x_pos / 100.0, y: y_pos / 100.0})
                
                // Add ID
                .insert(ID {id: i});
    }
}

fn boid_movement_system(mut query: Query<(&mut Transform, &mut Velocity, &ID)>, win_size: Res<WinSize>, boid_settings: Res<BoidSettings>) {

    for (mut transform, mut velocity, id) in query.iter_mut() {
        
        if velocity.length_squared() > boid_settings.boid_max_speed * boid_settings.boid_max_speed  {
            let velocity_vec = velocity.normalize() * boid_settings.boid_max_speed;
            velocity.x = velocity_vec.x;
            velocity.y = velocity_vec.y;
        }

        // Change boid position by velocity
        let translation = &mut transform.translation;
        translation.x += velocity.x;
        translation.y += velocity.y;

        // If the boid goes off screen, loop to other side
        if translation.x > win_size.w/2.0 || translation.x < -win_size.w/2.0 {
            translation.x = -translation.x
        }
        if translation.y > win_size.h/2.0 || translation.y < -win_size.h/2.0 {
            translation.y = -translation.y
        }
    }
}

fn boid_cohesion_system(transform_query: Query<(&Transform, &ID)>, mut velocity_query: Query<(&mut Velocity, &Transform, &ID)>, boid_settings: Res<BoidSettings>) {
    
    // Loop through all boids and get their position and velocity
    for (mut velocity, first_position, id) in velocity_query.iter_mut() {
        let mut center_of_mass = Vec2::new(0., 0.);
        let mut count = 0;
        let first_position = first_position.translation;
        let first_boid_position = Vec2::new(first_position.x, first_position.y);

        // Go through all boids again and get their position
        for (second_position, second_id) in transform_query.iter() {
            let second_position = second_position.translation;
            let second_boid_position = Vec2::new(second_position.x, second_position.y);

            // Find distance between first boid and second
            let distance_sqrd = (first_boid_position - second_boid_position).length_squared();

            //  If the boids are within view distance and are not occupying the same space add to the center of mass
            if distance_sqrd < boid_settings.boid_view_distance_sqrd && id != second_id {
                center_of_mass += second_boid_position;
                count += 1;
            }
        }

        //  If there were no boids in view distance move on
        if count == 0 {
            continue;
        }

        // Find the new velocity for the first boid
        center_of_mass /= count as f32;

        let new_velocity = (first_boid_position - center_of_mass) / 100.;
        
        velocity.x = new_velocity.x;
        velocity.y = new_velocity.y;
    }
}