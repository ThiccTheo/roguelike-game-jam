use {
    crate::core::{
        debug::add_debug_name,
        graphics::{AsciiTextureAtlas, SPRITE_DIMENSIONS},
    },
    bevy::prelude::*,
};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tiles);
    }
}

#[derive(Component)]
pub struct Tile;

fn spawn_tiles(mut cmds: Commands, tex_atlas: Res<AsciiTextureAtlas>) {
    for y in 0..20 {
        for x in 0..20 {
            if y == 0 || y == 19 || x == 0 || x == 19 {
                let mut tile = cmds.spawn(Tile);

                tile.insert(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: '#' as usize,
                        ..default()
                    },
                    texture_atlas: tex_atlas.0.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * SPRITE_DIMENSIONS.x,
                        y as f32 * SPRITE_DIMENSIONS.y,
                        1.,
                    ),
                    ..default()
                });

                #[cfg(debug_assertions)]
                add_debug_name(&mut tile, "Tile");
            }
        }
    }
}
