use {
    crate::core::{
        debug::add_debug_name,
        graphics::{AsciiTextureAtlas, SPRITE_DIMENSIONS},
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

const PLAYER_MOVEMENT_SPEED: f32 = 100.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut cmds: Commands, tex_atlas: Res<AsciiTextureAtlas>) {
    let mut player = cmds.spawn(Player);

    player
        .insert(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: '@' as usize,
                ..default()
            },
            texture_atlas: tex_atlas.0.clone(),
            transform: Transform::from_xyz(40., 40., 3.),
            ..default()
        })
        .insert(Collider::cuboid(
            SPRITE_DIMENSIONS.x / 2.,
            SPRITE_DIMENSIONS.y / 2.,
        ))
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(1.),
            ..default()
        });

    #[cfg(debug_assertions)]
    add_debug_name(&mut player, "Player");
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_qry: Query<&mut KinematicCharacterController, With<Player>>,
) {
    let dt = time.delta_seconds();
    let offset = dt * PLAYER_MOVEMENT_SPEED;
    let mut translation = Vec3::ZERO;

    let Ok(mut char_controller) = player_qry.get_single_mut() else { return };

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
    char_controller.translation = Some(translation.truncate());
}
