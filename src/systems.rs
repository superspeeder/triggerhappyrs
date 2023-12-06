use bevy::log::info;
use bevy::prelude::{Camera2dBundle, Commands, OrthographicProjection, Query, Res, State, Time};
use bevy::render::camera::ScalingMode;
use crate::components::{CameraMarker, DebugName};
use crate::constants::{RENDER_HEIGHT, RENDER_WIDTH};
use crate::GameState;

pub fn create_camera(id: usize) -> (Camera2dBundle, CameraMarker) {
    (Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: RENDER_WIDTH as f32,
                height: RENDER_HEIGHT as f32,
            },
            ..Default::default()
        },
        ..Default::default()
    }, CameraMarker(id))
}


pub fn on_startup(mut commands: Commands) {
    info!("Hello World!");

    commands.spawn(DebugName::new("Jim"));
    commands.spawn((create_camera(0), DebugName::new("Camera-0")));
}

pub fn on_update(time: Res<Time>, state: Res<State<GameState>>, query: Query<&DebugName>) {
    info!("State: {:?}", state);
    info!("Time: {:?}", time.elapsed());

    for debug_name in &query {
        info!("Found entity: {}", debug_name);
    }
}