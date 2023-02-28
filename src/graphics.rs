use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_startup_system_to_stage(StartupStage::PreStartup, load_textures)
            .add_startup_system(spawn_camera);
    }
}

const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut cmds: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.projection.scale /= 3.;
    cmds.spawn(cam).insert(MainCamera);
}

#[derive(Resource)]
pub struct AsciiSpriteSheet(pub Handle<TextureAtlas>);

fn load_textures(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let ascii = assets.load(format!("images/cp437_gray_black.png"));
    let atlas = TextureAtlas::from_grid(ascii, Vec2::new(8., 16.), 256 / 8, 388 / 16, None, None);
    let handle = atlases.add(atlas);
    cmds.insert_resource(AsciiSpriteSheet(handle));
}
