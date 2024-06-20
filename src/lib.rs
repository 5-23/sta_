mod chunk;
mod player;
use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    log::info!("loaded game plugin");
}
