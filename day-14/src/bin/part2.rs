use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::inspector_options::std_options::NumberDisplay;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

const INPUT: &str = include_str!("../input.txt");
const GRID_WIDTH: f32 = 101.0;
const GRID_HEIGHT: f32 = 103.0;
const SIZE: f32 = 8.0;
const WINDOW_WIDTH: f32 = GRID_WIDTH * SIZE;
const WINDOW_HEIGHT: f32 = GRID_HEIGHT * SIZE;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    #[inspector(min = 0.0, max = 10000.0, display = NumberDisplay::Slider, speed = 1.0)]
    tick: f32,
    #[inspector(min = 0.25, max = 100.0, display = NumberDisplay::Slider)]
    speed: f32,
    paused: bool,
    reverse: bool,
    smooth: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            tick: 6370.0,
            speed: 1.0,
            paused: true,
            reverse: false,
            smooth: true,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [WINDOW_WIDTH, WINDOW_HEIGHT].into(),
                title: "Advent of code 2024 - day 14 - part 2".to_string(),
                ..default()
            }),
            ..default()
        }))
        // if you don't use the `quick` plugins you need to add the `EguiPlugin` and the default inspector settings yourself
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        // insert and register resource
        .init_resource::<Configuration>()
        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(5)))
        .register_type::<Configuration>()
        .add_systems(Startup, (spawn_camera, spawn_robots))
        .add_systems(Update, sync_robot_transforms)
        .add_systems(Update, auto_tick)
        .add_systems(Update, update_speed)
        .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        .run();
}

fn sync_robot_transforms(
    mut query: Query<(&mut Robot, &mut Transform)>,
    config: Res<Configuration>,
) {
    let scaling = Vec3::new(SIZE, SIZE, 1.0);
    let tick = if config.smooth {
        config.tick
    } else {
        config.tick.trunc()
    };

    for (mut robot, mut transform) in &mut query {
        let position = robot.simulation_at(tick);
        transform.translation = position * scaling;
    }
}

fn auto_tick(mut config: ResMut<Configuration>, time: Res<Time<Virtual>>) {
    if config.paused {
        return;
    }
    if config.reverse {
        config.tick -= time.delta_secs();
    } else {
        config.tick += time.delta_secs();
    }
}

fn spawn_camera(mut commands: Commands) {
    let center = Vec3::new(WINDOW_WIDTH * 0.5, WINDOW_HEIGHT * 0.5, 0.0);
    commands.spawn((
        Camera2d,
        Transform::from_translation(center).looking_at(center, Vec3::NEG_Y),
    ));
}

fn spawn_robots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let square = meshes.add(Rectangle::new(SIZE, SIZE));

    for (i, robot) in get_robots().enumerate() {
        commands.spawn((
            Mesh2d(square.clone()),
            MeshMaterial2d(materials.add(Color::hsl(3.6 * i as f32, 0.75, 0.75))),
            Transform::from_xyz(robot.position.x * SIZE, robot.position.y * SIZE, 0.0),
            robot,
        ));
    }
}

#[derive(Debug, Component)]
struct Robot {
    position: Vec3,
    velocity: Vec3,
}

impl Robot {
    fn simulation_at(&mut self, tick: f32) -> Vec3 {
        (self.position + self.velocity * tick).rem_euclid(Vec3::new(GRID_WIDTH, GRID_HEIGHT, 1.0))
    }
}

fn get_robots() -> impl Iterator<Item = Robot> {
    INPUT.lines().map(|line| {
        let line = line.strip_prefix("p=").unwrap();
        let (position, velocity) = line.split_once(" v=").unwrap();
        let (x, y) = position.split_once(',').unwrap();
        let x = x.parse::<f32>().unwrap();
        let y = y.parse::<f32>().unwrap();
        let position = Vec3::new(x, y, 0.0);
        let (x, y) = velocity.split_once(',').unwrap();
        let x = x.parse::<f32>().unwrap();
        let y = y.parse::<f32>().unwrap();
        let velocity = Vec3::new(x, y, 0.0);
        Robot { position, velocity }
    })
    // .take(1)
}

fn update_speed(mut time: ResMut<Time<Virtual>>, config: Res<Configuration>) {
    time.set_relative_speed(config.speed.max(0.25));
    if config.paused && !time.is_paused() {
        time.pause();
    } else if !config.paused && time.is_paused() {
        time.unpause();
    }
}
