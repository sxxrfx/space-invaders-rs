use crate::{
    components::{Enemy, SpriteSize, Laser, Movable, FromEnemy, Velocity},
    EnemyCount, GameTextures, WinSize, ENEMY_MAX, ENEMY_SIZE, SPRITE_SCALE, ENEMY_LASER_SIZE, enemy_movement_system,
};
use bevy::{prelude::*, time::FixedTimestep, ecs::schedule::ShouldRun};
use rand::{thread_rng, Rng};

use self::formation::FormationMaker;

pub mod formation;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FormationMaker::default())
        .add_system_set(
            SystemSet::new()
            // .with_run_criteria(step)
            .with_run_criteria(FixedTimestep::step(1.))
            .with_system(enemy_spawn_system),
        )
        .add_system_set(
        SystemSet::new()
            .with_run_criteria(enemy_fire_criteria)
            .with_system(enemy_fire_system)
        )
        .add_system(enemy_movement_system);
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1. / 60.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }

}
fn enemy_spawn_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    mut formation_maker: ResMut<FormationMaker>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        // get formation and start x/y
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;

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
            .insert(formation)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
){
    for enemy_tf in enemy_query.iter() {

        let (x, y) = (enemy_tf.translation.x, enemy_tf.translation.y);

        commands
            .spawn_bundle(SpriteBundle{
                texture: game_textures.enemy_laser.clone(),
                transform: Transform{
                    translation: Vec3::new(x, y-15., 0.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable{ auto_despawn: true})
            .insert(Velocity{ x: 0. ,y: -1.});
    }

}