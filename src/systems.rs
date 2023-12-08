use std::f32::consts::PI;
use bevy::log::info;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, Assets, Camera2dBundle, Color, ColorMaterial, Commands, DetectChanges, Mesh, OrthographicProjection, Query, Res, ResMut, shape, State, Time, Transform, Vec3, With, DetectChangesMut};
use bevy::sprite::MaterialMesh2dBundle;
use crate::components::{DebugName, FunnyCircle};

pub fn on_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Hello World!");

    commands.spawn(DebugName::new("Jim"));
    commands.spawn((DebugName::new("Circle"), MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    }, FunnyCircle(0., 1.)));

    commands.spawn((DebugName::new("Circle-2"), MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::new(Vec2 { x: 50.0, y: 50.0 }).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    }, FunnyCircle(0., 1.)));

    commands.spawn((DebugName::new("Circle"), MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    }, FunnyCircle(PI, 1.)));

    commands.spawn((DebugName::new("Circle-2"), MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::new(Vec2 { x: 50.0, y: 50.0 }).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    }, FunnyCircle(PI, 1.)));

    commands.spawn((DebugName::new("Circle-3"), MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(14.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    }));
}

pub fn move_circle(time: Res<Time>, mut query: Query<(&mut Transform, &FunnyCircle)>) {
    for (mut transform, funny_circle) in &mut query {
        let n = time.elapsed_seconds() * funny_circle.1 + funny_circle.0;
        transform.translation = Vec3 { x: -64.0 * n.cos(), y: 64.0 * n.sin(), z: transform.translation.z};
        transform.rotation = Quat::from_rotation_z(-n);
    }
}
