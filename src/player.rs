use crate::GameState;
use crate::TextureAssets;
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
    textures: Res<TextureAssets>,
    texture_assets: Res<Assets<Texture>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!(
        "spawning texture; handle status: {:?}, asset is some? {}",
        asset_server.get_load_state(textures.bevy.id),
        texture_assets.get(textures.bevy.clone()).is_some()
    );
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(textures.bevy.clone().into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player);
}
