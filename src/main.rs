mod core {
    pub mod debug;
    pub mod graphics;
}
mod world {
    pub mod player;
    pub mod tile;
}

use {
    crate::core::{debug::DebugPlugin, graphics::GraphicsPlugin},
    bevy::{prelude::*, render::texture::ImageSampler},
    world::{player::PlayerPlugin, tile::TilePlugin},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSampler::nearest_descriptor(),
        }))
        .add_plugin(DebugPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TilePlugin)
        .run();
}
