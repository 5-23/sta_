use bevy::prelude::*;
use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    math::Vec2,
    prelude::{Component, Res},
    sprite::{Sprite, SpriteBundle},
};
use bevy_inspector_egui::prelude::*;
use bevy_xpbd_2d::{
    components::{GravityScale, RigidBody},
    prelude::Collider,
    prelude::*,
};

const HITBOX: (f32, f32) = (40., 40.);

#[derive(Component, Reflect, InspectorOptions)]
pub struct Player {
    on_ground: bool,
    #[inspector(min = 0.0, max = 100.0)]
    gas: f32,
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Player>();
        app.add_systems(Startup, setup)
            .add_systems(Update, (movement, physic, hit, camera_movement));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Player"),
        Player {
            on_ground: false,
            gas: 100.,
        },
        RigidBody::Dynamic,
        GravityScale(40.),
        Rotation::from_degrees(90.0),
        Collider::rectangle(HITBOX.0, HITBOX.1),
        ShapeCaster::new(
            Collider::rectangle(HITBOX.0, HITBOX.1),
            Vec2::ZERO,
            0.0,
            Direction2d::X,
        ),
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

fn movement(
    mut q: Query<(&mut Player, &mut Transform, &mut LinearVelocity)>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut player, mut transform, mut val)) = q.get_single_mut() {
        transform.rotation = Quat::from_axis_angle(Vec3::Z, 0.);
        log::info!("val-y: {:?}", val.y);

        if key.pressed(KeyCode::ArrowLeft) || key.pressed(KeyCode::KeyA) {
            val.x -= 5. * time.delta_seconds() * 100.;
        }

        if key.pressed(KeyCode::ArrowRight) || key.pressed(KeyCode::KeyD) {
            val.x += 5. * time.delta_seconds() * 100.;
        }

        if key.pressed(KeyCode::ArrowUp) || key.pressed(KeyCode::KeyW) {
            // if player.on_ground {
            //     val.y += 100. * time.delta_seconds() * 100.;
            // }
            if player.gas - 0.5 >= 0. {
                val.y += 10. * time.delta_seconds() * 100.;
                player.gas -= 0.5;
            } else {
                player.gas = 0.;
            }
        }
    } else {
        return;
    }
}

fn physic(mut q: Query<(&Player, &mut Transform, &mut LinearVelocity)>) {
    if let Ok((_, mut transform, mut physic)) = q.get_single_mut() {
        transform.rotation = Quat::from_axis_angle(Vec3::Z, 0.);
        if physic.y >= 200. {
            physic.y = 200.;
        }
        if physic.y <= -400. {
            physic.y = -400.;
        }
        // if physic.x <= -5. {
        //     physic.x = -5.;
        // }
        // if physic.x >= 5. {
        //     physic.x = 5.;
        // }
    } else {
        return;
    }
}

fn hit(mut q: Query<(&mut Player, &ShapeHits)>) {
    if let Ok((mut player, hits)) = q.get_single_mut() {
        player.on_ground = hits.iter().any(|hit| {
            if hit.normal1.y > 0.0 || hit.normal2.y > 0.0 {
                true
            } else {
                false
            }
        });
    }
}

fn camera_movement(
    player_q: Query<&Transform, With<Player>>,
    mut camera_q: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_q.get_single() {
        if let Ok(mut camera_transform) = camera_q.get_single_mut() {
            camera_transform.translation.x +=
                (player_transform.translation.x - camera_transform.translation.x) / 20.
                    * time.delta_seconds()
                    * 100.;
            camera_transform.translation.y +=
                (player_transform.translation.y - camera_transform.translation.y) / 20.
                    * time.delta_seconds()
                    * 100.;
        }
    } else {
        return;
    }
}
