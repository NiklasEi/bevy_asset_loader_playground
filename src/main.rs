mod player;

use crate::player::PlayerPlugin;
#[cfg(feature = "working")]
#[cfg(not(feature = "broken"))]
use bevy::audio::{Audio, AudioSource};
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
#[cfg(feature = "broken")]
#[cfg(not(feature = "working"))]
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource};

fn main() {
    let mut app = App::build();
    AssetLoader::new(GameState::Loading, GameState::Menu)
        .with_collection::<AudioAssets>()
        .with_collection::<TextureAssets>()
        .build(&mut app);

    app.add_state(GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(play_audio.system()));

    #[cfg(feature = "broken")]
    #[cfg(not(feature = "working"))]
    app.add_plugin(AudioPlugin);

    app.run();
}

fn play_audio(assets: Res<AudioAssets>, audio: Res<Audio>, mut state: ResMut<State<GameState>>) {
    println!("play audio");
    audio.play(assets.flying.clone());
    state.set(GameState::Playing).unwrap();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Menu,
    Playing,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Texture>,
}
