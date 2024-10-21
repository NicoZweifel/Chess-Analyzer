use bevy::prelude::*;

mod engine;
mod fen;
mod file_drop;
mod paste_clipboard;
mod pgn;
mod startup;

pub(crate) use engine::*;
pub(crate) use fen::*;
pub(crate) use file_drop::*;
pub(crate) use paste_clipboard::*;
pub(crate) use pgn::*;
pub(crate) use startup::*;

use crate::History;

pub struct AnalysisPlugin;

impl Plugin for AnalysisPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FenEvent>()
            .add_event::<PgnEvent>()
            .add_event::<EngineEvent>()
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                (
                    send_to_engines,
                    check_engines,
                    clipboard_paste,
                    fen,
                    pgn::<History>,
                    file_drop,
                ),
            );
    }
}
