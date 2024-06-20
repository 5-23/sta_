use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    math::Vec2,
    prelude::{Component, Res, *},
    sprite::{Sprite, SpriteBundle},
};
use bevy_xpbd_2d::{
    components::{GravityScale, MassPropertiesBundle, RigidBody},
    prelude::Collider,
    prelude::*,
};

use crate::{chunk::spawn_chunk, System};

const HITBOX: (f32, f32) = (100., 100.);

#[derive(Component)]
pub struct Player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}

fn setup(mut gizmos: Gizmos, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Player"),
        RigidBody::Kinematic,
        Collider::rectangle(HITBOX.0, HITBOX.1),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(HITBOX.0, HITBOX.1)),
                ..Default::default()
            },
            texture: asset_server.load("textures/player.png"),
            ..Default::default()
        },
        Player,
    ));
}

fn update(
    mut gizmos: Gizmos,
    mut q: Query<(&Name, &mut LinearVelocity, &Transform)>,
    time: Res<Time>,
) {
    // log::info!("Player update");
    for (&ref name, mut vel, transform) in &mut q {
        vel.y -= 10. * time.delta_seconds() * 100.;
        #[cfg(debug_assertions)]
        gizmos.rect_2d(
            transform.translation.xy(),
            0.,
            Vec2::new(HITBOX.0, HITBOX.1),
            Color::RED,
        );

        // log::info!("{name:?}: {vel:?}{}", time.delta_seconds());
    }
    // for entity in &query {
    //     info!("{:?} is colliding with the following entities", entity);
    // }
}
