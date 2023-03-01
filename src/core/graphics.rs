use {super::debug::add_debug_name, bevy::prelude::*};

const BACKGROUND_COLOR: Color = Color::BLACK;
const CAMERA_ZOOM_FACTOR: f32 = 3.;
const ASCII_SHEET_DIMENSIONS: Vec2 = Vec2::new(256., 388.);
pub const SPRITE_DIMENSIONS: Vec2 = Vec2::new(8., 16.);

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(Msaa { samples: 1 })
            .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_sheets)
            .add_startup_system(spawn_main_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_main_camera(mut cmds: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.projection.scale /= CAMERA_ZOOM_FACTOR;

    let mut cam = cmds.spawn(cam);
    cam.insert(MainCamera);

    #[cfg(debug_assertions)]
    add_debug_name(&mut cam, "Main Camera");
}

#[derive(Resource)]
pub struct AsciiTextureAtlas(pub Handle<TextureAtlas>);

fn load_ascii_sheets(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let img = assets.load(format!("images/cp437_gray_black.png"));
    let tex_atlas = TextureAtlas::from_grid(
        img,
        SPRITE_DIMENSIONS,
        (ASCII_SHEET_DIMENSIONS.x / SPRITE_DIMENSIONS.x) as usize,
        (ASCII_SHEET_DIMENSIONS.y / SPRITE_DIMENSIONS.y) as usize,
        None,
        None,
    );
    let handle = tex_atlases.add(tex_atlas);
    cmds.insert_resource(AsciiTextureAtlas(handle));
}
