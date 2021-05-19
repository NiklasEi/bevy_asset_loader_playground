mod player;

use bevy::prelude::*;
use bevy_asset_loader::{AssetLoaderPlugin, AssetCollection};
use crate::player::PlayerPlugin;

fn main() {
    let mut app = App::build();

    app.add_state(GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(AssetLoaderPlugin::<GameAssets, _>::new(
        GameState::Loading,
        GameState::Menu,
    ))
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(play_audio.system())).run();
}

fn play_audio(assets: Res<GameAssets>, audio: Res<Audio>, mut state: ResMut<State<GameState>>) {
    audio.play(assets.flying.clone());
        state.set(GameState::Playing).unwrap();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Menu,
    Playing
}

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Texture>,
}
