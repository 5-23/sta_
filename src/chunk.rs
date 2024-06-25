use bevy::prelude::*;
use bevy_xpbd_2d::{components::RigidBody, prelude::Collider};
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
        app.add_systems(Update, debug_chunk);
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
                .with_children(|_| {});
        }
    }
}

pub fn debug_chunk(
    mut gizmos: Gizmos,
    mut chunk_query: Query<(Entity, &Children, &Chunk), With<Chunk>>,
    mut commands: Commands,
) {
    for (entity, child, chunk) in chunk_query.iter_mut() {
        let entity = commands.get_entity(entity);
        if entity.is_none() {
            continue;
        }
        let mut entity = entity.unwrap();
        if child.len() == 0 {
            let perlin = Perlin::new(SEED.clone());

            entity.with_children(|par| {
                for i in 0..(CHUNK_SIZE as isize) {
                    for j in 0..(CHUNK_SIZE as isize) {
                        let val = perlin.get([
                            (chunk.x * CHUNK_SIZE + i as f32) as f64 / NOISE_SCALE,
                            (chunk.y * CHUNK_SIZE + j as f32) as f64 / NOISE_SCALE,
                        ]);
                        if val >= 0.2 {
                            par.spawn((
                                Name::new("Block"),
                                RigidBody::Static,
                                Collider::rectangle(BLOCK_SIZE, BLOCK_SIZE),
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                                        color: Color::hex("14368C").unwrap_or(Color::RED),
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
    }
}

pub fn render_chunk(
    mut gizmos: Gizmos,
    mut chunk_query: Query<(Entity, &Children, &Chunk), With<Chunk>>,
    mut player: Query<&Transform, (With<Player>, Without<Chunk>)>,
    mut commands: Commands,
) {
    let mut player = player.get_single_mut().unwrap();
    for (entity, child, chunk) in chunk_query.iter_mut() {
        let player_tile_x = player.translation.x / (CHUNK_SIZE * BLOCK_SIZE);
        let player_tile_y = player.translation.y / (CHUNK_SIZE * BLOCK_SIZE);
        log::info!("{}", chunk.x - player_tile_x);
        if (chunk.x - player_tile_x) <= -3.
            || (chunk.x - player_tile_x) >= 3.
            || (chunk.y - player_tile_y) <= -3.
            || (chunk.y - player_tile_y) >= 3.
        {
            commands.get_entity(entity).unwrap().despawn()
        } else {
            gizmos.rect_2d(
                Vec2::new(
                    chunk.x * CHUNK_SIZE * BLOCK_SIZE,
                    chunk.y * CHUNK_SIZE * BLOCK_SIZE,
                ),
                0.,
                Vec2::new(CHUNK_SIZE * BLOCK_SIZE, CHUNK_SIZE * BLOCK_SIZE),
                Color::GREEN,
            );
        }
    }
}
