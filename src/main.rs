use bevy::prelude::*;

const PLAYER_SPRITE: &str = "rusticon.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 720;
const SPRITE_SCALE: f32 = 0.5;

pub struct  WinSize {
    pub w:f32,
    pub h:f32,
}

pub struct GameTextures {
    player: Handle<Image>
}


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))    
        .insert_resource(WindowDescriptor{
            title: "Space Invaders!".to_string(),
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        .run();
}


fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
){
    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WinSize {
        w: win_w,
        h: win_h
    };

    // add WinSize resource
    commands.insert_resource(win_size);

    // add GameTextures resource
    let game_textures = GameTextures{
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures);

    // position of the window : OPTIONAL
    window.set_position(IVec2::new((1920-WINDOW_WIDTH)/2, (1080- WINDOW_HEIGHT)/2));

    // camera
    commands.spawn_bundle(Camera2dBundle::default());

}

fn player_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>,
) {

    // get player sprites
    let bottom = -win_size.h / 2.; 
    commands.spawn_bundle(SpriteBundle{
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform{
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1/ 2. * SPRITE_SCALE + 5., 10.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}