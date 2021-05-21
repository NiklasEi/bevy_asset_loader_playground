mod player;

use crate::player::PlayerPlugin;
use bevy::asset::LoadState;
#[cfg(feature = "working")]
#[cfg(not(feature = "broken"))]
use bevy::audio::{Audio, AudioSource};
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoaderPlugin};
#[cfg(feature = "broken")]
#[cfg(not(feature = "working"))]
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource};

fn main() {
    let mut app = App::build();

    app.add_state(GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        // .add_plugin(AssetLoaderPlugin::<GameAssets, _>::new(
        //     GameState::Loading,
        //     GameState::Playing,
        // ))
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(play_audio.system()))
        .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load.system()))
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check.system()));

    #[cfg(feature = "broken")]
    #[cfg(not(feature = "working"))]
    app.add_plugin(AudioPlugin);

    app.run();
}

fn play_audio(assets: Res<GameAssets>, audio: Res<Audio>, mut state: ResMut<State<GameState>>) {
    audio.play(assets.flying.clone());
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Menu,
    Playing,
}

// #[derive(AssetCollection)]
pub struct GameAssets {
    // #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Texture>,
    // #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

struct Handles {
    handles: Vec<HandleUntyped>,
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handles = vec![];
    handles.push(asset_server.load_untyped("textures/bevy.png"));
    handles.push(asset_server.load_untyped("audio/flying.ogg"));
    commands.insert_resource(Handles { handles });
}

fn check(
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    handles: Res<Handles>,
    asset_server: Res<AssetServer>,
) {
    if LoadState::Loaded == asset_server.get_group_load_state(handles.handles.iter().map(|h| h.id))
    {
        commands.insert_resource(GameAssets {
            texture_bevy: asset_server.get_handle("textures/bevy.png"),
            flying: asset_server.get_handle("audio/flying.ogg"),
        });
        println!("loaded");
        state.set(GameState::Playing).unwrap();
    }
}
