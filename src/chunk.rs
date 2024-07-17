use avian2d::{
    collision::Collider,
    prelude::{CoefficientCombine, Restitution, RigidBody, SpeculativeMargin},
};
use bevy::{color::palettes::tailwind, prelude::*};
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};
use lazy_static::lazy_static;
use noise::{NoiseFn, Perlin};

use crate::player::Player;

const BLOCK_SIZE: f32 = 40.;
const CHUNK_SIZE: f32 = 8.;
const NOISE_SCALE: f64 = 30.7;
// static mut SEED: u32 = 0;
lazy_static! {
    static ref SEED: u32 = rand::random::<u32>();
}
#[derive(Component)]
struct NoneEntity;

#[derive(Component)]
pub struct Chunk {
    x: f32,
    y: f32,
}

pub struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chunk);
        // app.add_systems(Startup, spawn_block);
        app.add_systems(Update, render_chunk);
        // app.add_systems(Update, debug_chunk);
    }
}

pub fn spawn_chunk(_gizmos: Gizmos, mut commands: Commands, _asset_server: Res<AssetServer>) {
    for y in -2..3 {
        for x in -2..3 {
            commands
                .spawn((
                    Name::new("Chunk"),
                    Chunk {
                        x: x as f32,
                        y: y as f32,
                    },
                    SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(
                            x as f32 * CHUNK_SIZE * BLOCK_SIZE,
                            y as f32 * CHUNK_SIZE * BLOCK_SIZE,
                            0.,
                        )),
                        ..Default::default()
                    },
                ))
                .with_children(|p| {
                    p.spawn(NoneEntity);
                });
        }
    }
}

// pub fn debug_chunk(
//     gizmos: Gizmos,
//     mut chunk_query: Query<(Entity, &Children, &Chunk)>,
//     mut commands: Commands,
// ) {
//     for (entity, child, chunk) in chunk_query.iter_mut() {
//         let entity = commands.get_entity(entity);
//         if entity.is_none() {
//             return;
//         }
//     }
// }

pub fn render_chunk(
    mut gizmos: Gizmos,
    mut chunk_query: Query<(Entity, &Children, &Chunk), With<Chunk>>,
    mut player: Query<&Transform, (With<Player>, Without<Chunk>)>,
    mut commands: Commands,
) {
    let player = player.get_single_mut().unwrap();
    // log::info!("chunk: {:?}", chunk_query.iter().len());
    for (entity, child, chunk) in chunk_query.iter_mut() {
        let cx = chunk.x - player.translation.x / (CHUNK_SIZE * BLOCK_SIZE);
        let cy = chunk.y - player.translation.y / (CHUNK_SIZE * BLOCK_SIZE);
        let n = 3.;
        let mut e = commands.get_entity(entity).unwrap();
        if cx < -n || cx >= n || cy < -n || cy >= n {
            e.despawn_descendants();
            e.despawn();

            if cx < -n {
                commands
                    .spawn((
                        Name::new("Chunk"),
                        Chunk {
                            x: chunk.x + 5.,
                            y: chunk.y,
                        },
                        SpriteBundle {
                            transform: Transform::from_translation(Vec3::new(
                                (chunk.x + 5.) * CHUNK_SIZE * BLOCK_SIZE,
                                chunk.y * CHUNK_SIZE * BLOCK_SIZE,
                                0.,
                            )),
                            ..Default::default()
                        },
                    ))
                    .with_children(|p| {
                        p.spawn(NoneEntity);
                    });
            }
            if cx >= n {
                commands
                    .spawn((
                        Name::new("Chunk"),
                        Chunk {
                            x: chunk.x - 5.,
                            y: chunk.y,
                        },
                        SpriteBundle {
                            transform: Transform::from_translation(Vec3::new(
                                (chunk.x - 5.) * CHUNK_SIZE * BLOCK_SIZE,
                                chunk.y * CHUNK_SIZE * BLOCK_SIZE,
                                0.,
                            )),
                            ..Default::default()
                        },
                    ))
                    .with_children(|p| {
                        p.spawn(NoneEntity);
                    });
            }
            if cy < -n {
                commands
                    .spawn((
                        Name::new("Chunk"),
                        Chunk {
                            x: chunk.x,
                            y: chunk.y + 5.,
                        },
                        SpriteBundle {
                            transform: Transform::from_translation(Vec3::new(
                                chunk.x * CHUNK_SIZE * BLOCK_SIZE,
                                (chunk.y + 5.) * CHUNK_SIZE * BLOCK_SIZE,
                                0.,
                            )),
                            ..Default::default()
                        },
                    ))
                    .with_children(|p| {
                        p.spawn(NoneEntity);
                    });
            }
            if cy >= n {
                commands
                    .spawn((
                        Name::new("Chunk"),
                        Chunk {
                            x: chunk.x,
                            y: chunk.y - 5.,
                        },
                        SpriteBundle {
                            transform: Transform::from_translation(Vec3::new(
                                chunk.x * CHUNK_SIZE * BLOCK_SIZE,
                                (chunk.y - 5.) * CHUNK_SIZE * BLOCK_SIZE,
                                0.,
                            )),
                            ..Default::default()
                        },
                    ))
                    .with_children(|p| {
                        p.spawn(NoneEntity);
                    });
            }
        } else {
            if child.len() == 1 {
                let perlin = Perlin::new(SEED.clone());

                e.with_children(|par| {
                    for i in 0..(CHUNK_SIZE as isize) {
                        for j in 0..(CHUNK_SIZE as isize) {
                            let val_block = perlin.get([
                                (chunk.x * CHUNK_SIZE + i as f32) as f64 / NOISE_SCALE,
                                (chunk.y * CHUNK_SIZE + j as f32) as f64 / NOISE_SCALE,
                            ]);

                            let val_ore = perlin.get([
                                (chunk.x * CHUNK_SIZE + i as f32) as f64 / 1.2,
                                (chunk.y * CHUNK_SIZE + j as f32) as f64 / 1.2,
                            ]);
                            if val_ore >= 0.7 && 0.11 > val_block && val_block >= 0.1 {
                                par.spawn((
                                    Name::new("Ore"),
                                    RigidBody::Static,
                                    Restitution::new(0.4)
                                        .with_combine_rule(CoefficientCombine::Multiply),
                                    Collider::rectangle(BLOCK_SIZE, BLOCK_SIZE),
                                    SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                                            color: Srgba::hex("A7DEFE")
                                                .unwrap_or(Srgba::RED)
                                                .into(),
                                            ..Default::default()
                                        },
                                        transform: Transform::from_translation(Vec3::new(
                                            i as f32 * BLOCK_SIZE - 140.,
                                            j as f32 * BLOCK_SIZE - 140.,
                                            0.,
                                        )),
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|p| {
                                    p.spawn(PointLight2dBundle {
                                        point_light: PointLight2d {
                                            radius: 200.0,
                                            intensity: 10.0,
                                            falloff: 10.,
                                            color: Srgba::hex("A7DEFE").unwrap().into(),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                            } else if val_block >= 0.2 {
                                par.spawn((
                                    Name::new("Block"),
                                    RigidBody::Static,
                                    Restitution::new(0.4)
                                        .with_combine_rule(CoefficientCombine::Multiply),
                                    Collider::rectangle(BLOCK_SIZE, BLOCK_SIZE),
                                    SpriteBundle {
                                        sprite: Sprite {
                                            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                                            color: Srgba::hex("14368C")
                                                .unwrap_or(Srgba::RED)
                                                .into(),
                                            ..Default::default()
                                        },
                                        transform: Transform::from_translation(Vec3::new(
                                            i as f32 * BLOCK_SIZE - 140.,
                                            j as f32 * BLOCK_SIZE - 140.,
                                            0.,
                                        )),
                                        ..Default::default()
                                    },
                                ));
                            }
                        }
                    }
                });
            }

            gizmos.rect_2d(
                Vec2::new(
                    chunk.x * CHUNK_SIZE * BLOCK_SIZE,
                    chunk.y * CHUNK_SIZE * BLOCK_SIZE,
                ),
                0.,
                Vec2::new(CHUNK_SIZE * BLOCK_SIZE, CHUNK_SIZE * BLOCK_SIZE),
                tailwind::GREEN_400,
            );
        }
    }
}
