use bevy::{
    app::{App, Startup},
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, Query, ResMut},
    },
    math::{primitives::Circle, Vec2},
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::Transform,
    DefaultPlugins,
};

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position,
}

impl BallBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(x, y)),
        }
    }
}

const BALL_SIZE: f32 = 5.;

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");

    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::rgb(1., 0., 0.));

    // `Assets::add` will load these into memory and return a
    // `Handle` (an ID) to these assets. When all references
    // to this `Handle` are cleaned up the asset is cleaned up.
    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    // Here we are using `spawn` instead of `spawn_empty`
    // followed by an `insert`. They mean the same thing,
    // letting us spawn many components on a new entity at once.
    commands.spawn((
        BallBundle::new(0., 0.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..Default::default()
        },
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    // Here we are iterating over the query to get the
    // components from our game world
    for (mut transform, position) in positionables.iter_mut() {
        transform.translation = position.0.extend(0.);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .add_systems(Startup, project_positions)
        .run();
}
