#![windows_subsystem = "windows"]

use bevy::{
    prelude::*,
    winit::{UpdateMode, WinitSettings},
};

mod game;
mod mainmenu;

use game::GamePlugin;
use mainmenu::MainMenuPlugin;

fn main() {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .insert_resource(ClearColor(Color::GRAY))
        .insert_state(AppState::MainMenu)
        .add_plugins((GamePlugin, MainMenuPlugin))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

//App state so we can keep track of if we're on mainmenu or playing
#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    MainMenu,
    Playing,
}
