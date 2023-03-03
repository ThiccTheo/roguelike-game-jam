use {
    crate::{
        core::{debug::add_debug_name, graphics::AsciiTextureAtlas, physics::Velocity},
        world::player::Player,
    },
    bevy::prelude::*,
    bevy_mouse_tracking_plugin::MousePosWorld,
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_bullet);
    }
}

#[derive(Component)]
pub struct Bullet;

fn spawn_bullet(
    mut cmds: Commands,
    mouse: Res<Input<MouseButton>>,
    tex_atlas: Res<AsciiTextureAtlas>,
    mouse_pos: Res<MousePosWorld>,
    player_qry: Query<&Transform, With<Player>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Ok(player_transform) = player_qry.get_single() else { return };
        let player_pos = player_transform.translation.truncate();
        let (delta_x, delta_y) = (mouse_pos.x - player_pos.x, mouse_pos.y - player_pos.y);
        let magnitude = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
        let dir = Vec2::new(delta_x / magnitude, delta_y / magnitude);

        let mut bullet = cmds.spawn(Bullet);

        bullet
            .insert(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 4,
                    ..default()
                },
                texture_atlas: tex_atlas.0.clone(),
                transform: Transform::from_xyz(player_pos.x, player_pos.y, 1.),
                ..default()
            })
            .insert(Velocity(dir * Vec2::splat(5.)));

        #[cfg(debug_assertions)]
        add_debug_name(&mut bullet, "Bullet");
    }
}
