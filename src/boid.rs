use bevy::prelude::*;
use rand::Rng;

use crate::{SPRITE_SCALE, GameTextures, WinSize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, boid_spawn_system);
    }
}

fn boid_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {

    for _i in 0..100 {
        
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
            });
    }
}
