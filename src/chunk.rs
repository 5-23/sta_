use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Collider;
const BLOCK_SIZE: f32 = 40.;
const CHUNK_SIZE: f32 = 8.;
static mut CHUNKS: Vec<Vec2> = vec![];

pub struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chunk);
        app.add_systems(Update, debug_chunk);
    }
}

pub fn spawn_chunk(mut gizmos: Gizmos, mut commands: Commands, asset_server: Res<AssetServer>) {
    for y in -5..5 {
        for x in -5..5 {
            unsafe {
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
