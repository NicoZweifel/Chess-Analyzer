use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod clear_indicators;
mod core;
mod drag_drop;
mod engine;
mod fen;
mod file_drop;
mod history;
mod paste_clipboard;
mod pgn;
mod select;
mod startup;
mod update;
mod utils;

pub(crate) use clear_indicators::*;
pub(crate) use core::*;
pub(crate) use drag_drop::*;
pub(crate) use engine::*;
pub(crate) use fen::*;
pub(crate) use file_drop::*;
pub(crate) use history::*;
pub(crate) use paste_clipboard::*;
pub(crate) use pgn::*;
pub(crate) use select::*;
pub(crate) use startup::*;
pub(crate) use update::*;

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
        ))
        .add_event::<DropEvent>()
        .add_event::<SelectEvent>()
        .add_event::<DragEvent>()
        .add_event::<DragEndEvent>()
        .add_event::<EngineEvent>()
        .add_event::<FenEvent>()
        .add_event::<PgnEvent>()
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                update,
                clear_indicators,
                drop.before(drag_end),
                select,
                drag,
                drag_end.after(drop),
                send_to_engines,
                check_engines,
                clipboard_paste,
                fen,
                pgn,
                back,
                forward,
                first,
                last,
                file_drop,
            ),
        )
        .run();
}
