#![allow(unused)]

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const TIME_STEP: f32 = 1. / 60.;

pub struct Materials {
    player_material: Handle<ColorMaterial>,
    laser_material: Handle<ColorMaterial>,
}
struct WinSize {
    height: f32,
    width: f32,
}
//Entity, Component, System, Resource

struct Player;
struct Laser;
struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(500.0)
    }
}

fn main() {
    App::build()
        //set color for game
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        //set window size
        .insert_resource(WindowDescriptor {
            title: "Boring Game".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let mut window = windows.get_primary_mut().unwrap();
    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //create the main resources
    commands.insert_resource(Materials {
        player_material: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser_material: materials.add(asset_server.load(LASER_SPRITE).into()),
    });

    //window size
    commands.insert_resource(WinSize {
        height: window.height(),
        width: window.width(),
    });
    //position windows
    window.set_position(IVec2::new(10, 10));
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<WinSize>) {
    //spawn a sprite
    let bottom = -win_size.height / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_material.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>,
) {
    if let Ok((speed, mut transform, _)) = query.single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
        transform.translation.x += speed.0 * TIME_STEP * dir;
    }
}
