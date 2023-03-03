mod core {
    pub mod debug;
    pub mod graphics;
    pub mod physics;
}
mod world {
    pub mod bullet;
    pub mod player;
    pub mod tile;
}
mod logic {
    pub mod shooting;
}

use {
    crate::core::{
        debug::DebugPlugin,
        graphics::{GraphicsPlugin, WINDOW_DIMENSIONS},
        physics::PhysicsPlugin,
    },
    bevy::prelude::*,
    bevy_mouse_tracking_plugin::prelude::MousePosPlugin,
    bevy_rapier2d::prelude::*,
    world::{bullet::BulletPlugin, player::PlayerPlugin, tile::TilePlugin},
    logic::shooting::ShootingPlugin,
};

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: WINDOW_DIMENSIONS.x,
                    height: WINDOW_DIMENSIONS.y,
                    ..default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin);

    #[cfg(debug_assertions)]
    //app.add_plugin(RapierDebugRenderPlugin::default());
    app.add_plugin(MousePosPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(PhysicsPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(TilePlugin)
        .add_plugin(ShootingPlugin)
        .run();
}
