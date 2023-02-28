use {
    crate::graphics::{AsciiSpriteSheet, MainCamera},
    bevy::prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut cmds: Commands, texture: Res<AsciiSpriteSheet>) {
    cmds.spawn(Player).insert(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: '@' as usize,
            ..default()
        },
        texture_atlas: texture.0.clone(),
        transform: Transform::from_xyz(0., 0., 1.),
        ..default()
    });
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut qry: Query<&mut Transform, Or<(With<Player>, With<MainCamera>)>>,
) {
    let dt = time.delta_seconds();
    let offset = dt * 100.;
    let mut translation = Vec3::ZERO;

    if keys.pressed(KeyCode::W) {
        translation.y += offset;
    }
    if keys.pressed(KeyCode::S) {
        translation.y -= offset;
    }
    if keys.pressed(KeyCode::A) {
        translation.x -= offset;
    }
    if keys.pressed(KeyCode::D) {
        translation.x += offset;
    }
    for mut transform in qry.iter_mut() {
        transform.translation += translation;
    }
}
