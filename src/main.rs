mod constants;
mod components;
mod systems;

extern crate log;
extern crate bevy;

use bevy::prelude::{App, ClearColor, DefaultPlugins, ImagePlugin, PluginGroup, Startup, States, Update};
use crate::constants::CLEAR_COLOR;
use crate::systems::{on_startup, on_update};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_state::<GameState>()
        .add_systems(Startup, on_startup)
        .add_systems(Update, on_update)
        .run();
}
