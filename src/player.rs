use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    math::Vec2,
    prelude::{Component, Res, *},
    sprite::{Sprite, SpriteBundle},
};
use bevy_xpbd_2d::{
    components::{GravityScale, RigidBody},
    prelude::Collider,
};

const HITBOX: (f32, f32) = (100., 100.);

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub position: Vec2,
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        RigidBody::Dynamic,
        GravityScale(2.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(HITBOX.0, HITBOX.1)),
                ..Default::default()
            },
            texture: asset_server.load("textures/player.png"),
            ..Default::default()
        },
    ));
}

fn update() {}
