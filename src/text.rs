use bevy::prelude::*;
use crate::{compontents::TextComponent};
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_text_system);
    }
}

fn setup_text_system(mut commands: Commands, asset_server: Res<AssetServer>) {    
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Nishant's\nBoids",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Medium.ttf"),
                    font_size: 50.0,
                    color: Color::rgba(0.5, 0.5, 0.5, 0.1),
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(TextComponent);
}