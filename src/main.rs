//! Showcases wireframe rendering.

mod elements;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    // commands
    //     .spawn()
    //     .insert(Paddle)
    //     .insert_bundle(SpriteBundle {
    //         transform: Transform {
    //             translation: Vec3::new(0.0, paddle_y, 0.0),
    //             scale: PADDLE_SIZE,
    //             ..default()
    //         },
    //         sprite: Sprite {
    //             color: PADDLE_COLOR,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .insert(Collider);
    //
    // // Ball
    // commands
    //     .spawn()
    //     .insert(Ball)
    //     .insert_bundle(SpriteBundle {
    //         transform: Transform {
    //             scale: BALL_SIZE,
    //             translation: BALL_STARTING_POSITION,
    //             ..default()
    //         },
    //         sprite: Sprite {
    //             color: BALL_COLOR,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));
    //
    // // Walls
    // commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    // commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    // commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    // commands.spawn_bundle(WallBundle::new(WallLocation::Top));
}
