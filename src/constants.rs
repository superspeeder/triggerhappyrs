use bevy::prelude::Color;
use bevy::render::view::Layer;

pub const RENDER_WIDTH: u32 = 640;
pub const RENDER_HEIGHT: u32 = 360;

pub const CLEAR_COLOR: Color = Color::rgb(0.0, 0.2, 0.2);

pub const OFFSCREEN_LAYER: Layer = 1;