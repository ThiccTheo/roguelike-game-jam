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
    bevy::prelude::*,
    world::{player::PlayerPlugin, tile::TilePlugin},
};

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin);

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(GraphicsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TilePlugin)
        .run();
}
