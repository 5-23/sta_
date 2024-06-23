use bevy::prelude::*;
use bevy_xpbd_2d::{
    components::{AngularDamping, LinearDamping, RigidBody},
    prelude::Collider,
};
use lazy_static::lazy_static;
use noise::{NoiseFn, Perlin};

const BLOCK_SIZE: f32 = 40.;
const CHUNK_SIZE: f32 = 8.;
const NOISE_SCALE: f64 = 30.7;
static mut CHUNKS: Vec<Vec2> = vec![];
// static mut SEED: u32 = 0;
lazy_static! {
    static ref SEED: u32 = rand::random::<u32>();
}
pub struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chunk);
        // app.add_systems(Startup, spawn_block);
        app.add_systems(Update, debug_chunk);
    }
}

pub fn spawn_chunk(_gizmos: Gizmos, mut commands: Commands, _asset_server: Res<AssetServer>) {
    for y in -2..3 {
        for x in -2..3 {
            unsafe {
                commands
                    .spawn((
                        Name::new("Chunk"),
                        SpriteBundle {
                            transform: Transform::from_translation(Vec3::new(
                                x as f32 * CHUNK_SIZE * BLOCK_SIZE,
                                y as f32 * CHUNK_SIZE * BLOCK_SIZE,
                                0.,
                            )),
                            ..Default::default()
                        },
                    ))
                    .with_children(|par| {
                        let perlin = Perlin::new(SEED.clone());
                        for i in 0..(CHUNK_SIZE as isize) {
                            for j in 0..(CHUNK_SIZE as isize) {
                                let val = perlin.get([
                                    (x * (CHUNK_SIZE) as isize + i) as f64 / NOISE_SCALE,
                                    (y * (CHUNK_SIZE) as isize + j) as f64 / NOISE_SCALE,
                                ]);
                                if val >= 0.2 {
                                    par.spawn((
                                        Name::new("Block"),
                                        LinearDamping(30.8),
                                        AngularDamping(10.6),
                                        RigidBody::Static,
                                        Collider::rectangle(BLOCK_SIZE, BLOCK_SIZE),
                                        SpriteBundle {
                                            sprite: Sprite {
                                                custom_size: Some(Vec2::new(
                                                    BLOCK_SIZE, BLOCK_SIZE,
                                                )),
                                                color: Color::hex("#14368C").unwrap(),
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
                CHUNKS.push(Vec2::new(x as f32, y as f32));
            }
        }
    }
}

pub fn debug_chunk(mut gizmos: Gizmos) {
    for chunk in unsafe { CHUNKS.iter() } {
        gizmos.rect_2d(
            chunk.clone() * BLOCK_SIZE * CHUNK_SIZE,
            0.,
            Vec2::new(BLOCK_SIZE * CHUNK_SIZE, BLOCK_SIZE * CHUNK_SIZE),
            Color::GREEN,
        );
    }
}
