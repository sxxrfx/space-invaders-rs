use std::collections::HashSet;

use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, ecs::entity};
use components::{Enemy, FromPlayer, Laser, Movable, SpriteSize, Velocity, ExplosionToSpawn, Explosion, ExplosionTimer};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
mod enemy;
mod player;

// Game Constants
const PLAYER_SPRITE: &str = "rusticon.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";

const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);
const EXPLOSION_LEN: usize = 16;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 720;
const SPRITE_SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
// END: Game Constants
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

pub struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Space Invaders!".to_string(),
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_system(player_laser_hit_enemy_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WinSize { w: win_w, h: win_h };

    // add WinSize resource
    commands.insert_resource(win_size);

    // create explosion texture atlas
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);
    let explosion = texture_atlases.add(texture_atlas);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion: explosion,
    };
    commands.insert_resource(game_textures);

    // position of the window : OPTIONAL
    window.set_position(IVec2::new(
        (1920 - WINDOW_WIDTH) / 2,
        (1080 - WINDOW_HEIGHT) / 2,
    ));

    // camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                // println!("->> despawn {entity:?}");
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    let despawned_entities: HashSet<Entity> = HashSet::new();
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                despawned_entities.insert(enemy_entity);
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);
                commands.spawn().insert(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}


fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures:Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
){
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        commands.spawn_bundle(SpriteSheetBundle{
            texture_atlas: game_textures.explosion.clone(),
            transform: Transform
            {
                translation: explosion_to_spawn.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Explosion)
        .insert(ExplosionTimer::default());

        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time:Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
){
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            sprite.index += 1;
            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}