use crate::GameAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system())
                .with_system(spawn_camera.system()),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("spawning");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(game_assets.texture_bevy.clone().into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player);
}