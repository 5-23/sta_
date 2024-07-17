mod chunk;
mod player;
use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};
use bevy_kira_audio::prelude::*;
use bevy_light_2d::light::AmbientLight2d;
use chunk::ChunkPlugin;
use player::PlayerPlugin;

pub struct GamePlugin;

#[derive(SystemSet, Debug, Hash, Eq, PartialEq, Clone)]
pub enum System {
    Player,
    Block,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(ChunkPlugin);
    }
}

fn setup(mut commands: Commands, audio: Res<Audio>, assets: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle::default(),
        AmbientLight2d {
            brightness: 0.1,
            ..default()
        },
    ));
    audio.play(assets.load("audio/bg.mp3")).looped();
}
