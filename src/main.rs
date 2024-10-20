use bevy::prelude::*;
use bevy_inspector_egui::egui::util::History;
use bevy_mod_picking::prelude::*;

mod clear_indicators;
mod core;
mod engine;
mod fen;
mod file_drop;
mod history;
mod paste_clipboard;
mod pgn;
mod picking;
mod play;
mod sound;
mod startup;
mod update;
mod utils;

pub(crate) use clear_indicators::*;
pub(crate) use core::*;
pub(crate) use engine::*;
pub(crate) use fen::*;
pub(crate) use file_drop::*;
pub(crate) use history::*;
pub(crate) use paste_clipboard::*;
pub(crate) use pgn::*;
pub(crate) use picking::*;
pub(crate) use play::*;
pub(crate) use sound::*;
pub(crate) use startup::*;
pub(crate) use update::*;

pub struct ChessPlugin;

pub struct AnalysisPlugin;
pub struct UiPlugin;

impl Plugin for AnalysisPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FenEvent>()
            .add_event::<PgnEvent>()
            .add_event::<EngineEvent>()
            .add_systems(
                Update,
                (
                    check_engines,
                    clipboard_paste,
                    fen,
                    pgn::<history::History>,
                    file_drop,
                ),
            );
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, (update, clear_indicators));
    }
}

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HistoryPlugin::<history::History, Game>::default(),
            PickingPlugin,
            AnalysisPlugin,
            UiPlugin,
        ));
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
