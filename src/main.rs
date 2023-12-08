mod constants;
mod components;
mod systems;
mod types;
mod pixel_perfect_rendering;

extern crate log;
extern crate bevy;

use bevy::prelude::{App, ClearColor, default, DefaultPlugins, ImagePlugin, Msaa, PluginGroup, Startup, Update, Window, WindowPlugin};
use bevy::render::view::RenderLayers;
use bevy::window::WindowMode;
use crate::pixel_perfect_rendering::PixelPerfectPlugin;
use crate::systems::{move_circle, on_startup};
use crate::types::GameState;

// const RS: (u32, u32) = (constants::RENDER_WIDTH, constants::RENDER_HEIGHT);
const RS: (u32, u32) = (320, 180);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    title: "Trigger Happy".to_string(),
                    ..default()
                }),
                ..default()
            }))
        .add_plugins(PixelPerfectPlugin::new(RS, constants::OFFSCREEN_LAYER,
                                             RenderLayers::all().without(constants::OFFSCREEN_LAYER)))
        .insert_resource(ClearColor(constants::CLEAR_COLOR))
        .insert_resource(Msaa::Off)
        .add_state::<GameState>()
        .add_systems(Startup, on_startup)
        .add_systems(Update, (move_circle,))
        .run();
}
