mod player;
use bevy::{
    app::{Plugin, Startup},
    prelude::{Camera2dBundle, Commands},
};
use player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
        app.add_plugins(PlayerPlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    log::info!("loaded game plugin");
}
