use {bevy::prelude::*, bevy_inspector_egui::quick::WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin);
        }
    }
}

pub fn debug_name(name: impl Into<String>, id: Entity) -> Name {
    Name::new(format!("{} | #{:?}", name.into(), id))
}
