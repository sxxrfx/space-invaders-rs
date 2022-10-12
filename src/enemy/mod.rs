use crate::{
    components::{Enemy, SpriteSize},
    GameTextures, WinSize, ENEMY_SIZE, SPRITE_SCALE,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    // compute the x/y
    let mut rng = thread_rng();

    let w_span = win_size.w / 2. - 100.;
    let h_span = win_size.h / 2. - 100.;

    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            texture: game_textures.enemy.clone(),
            ..Default::default()
        })
        .insert(Enemy)
        .insert(SpriteSize::from(ENEMY_SIZE));
}
