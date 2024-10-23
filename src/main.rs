use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod analysis;
mod audio;
mod core;
mod history;
mod picking;
mod play;
mod startup;
mod ui;

pub(crate) use analysis::AnalysisPlugin;
pub(crate) use audio::AudioPlugin;
pub(crate) use core::*;
pub(crate) use history::{History, HistoryPlugin};
pub(crate) use picking::PickingPlugin;
pub(crate) use play::*;
pub(crate) use startup::*;
pub(crate) use ui::UiPlugin;

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HistoryPlugin,
            PickingPlugin,
            AnalysisPlugin,
            UiPlugin,
            AudioPlugin,
        ))
        .add_systems(Startup, startup);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Chess"),
                        present_mode: bevy_window::PresentMode::AutoVsync,
                        resolution: (800., 800.).into(),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            DefaultPickingPlugins,
            ChessPlugin,
        ))
        .run();
}
