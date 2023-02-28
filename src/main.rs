mod graphics;
mod player;

use {
    bevy::{prelude::*, render::texture::ImageSampler},
    graphics::GraphicsPlugin,
    player::PlayerPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSampler::nearest_descriptor(),
        }))
        .add_plugin(GraphicsPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
