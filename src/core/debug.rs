use {
    bevy::{ecs::system::EntityCommands, prelude::*},
    bevy_inspector_egui::quick::WorldInspectorPlugin,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin);
    }
}

pub fn add_debug_name(entity: &mut EntityCommands, name: impl Into<String>) {
    entity.remove::<Name>();
    entity.insert(Name::new(format!("{} | #{:?}", name.into(), entity.id())));
}
