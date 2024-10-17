use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use drag::DragEvent;
use drag_end::DragEndEvent;
use drop::DropEvent;
use engine::EngineEvent;
use fen::FenEvent;
use pgn::PgnEvent;
use select::SelectEvent;
use shakmaty::{Bitboard, ByColor, ByRole};

mod back;
mod clear_indicators;
mod clipboard;
mod drag;
mod drag_end;
mod drop;
mod engine;
mod fen;
mod forward;
mod paste;
mod pgn;
mod select;
mod startup;
mod update;
mod utils;

#[derive(Component, Clone, Copy, Debug)]
struct Piece(shakmaty::Piece);

#[derive(Component, Clone, Copy, Debug)]
struct Indicator;

#[derive(Component, Clone, Copy, Debug)]
struct EngineMove;

#[derive(Component, Clone, Debug, Copy)]
struct Square {
    square: shakmaty::Square,
}

#[derive(Component, Clone, Debug)]
struct Game {
    board: Board,
    turn: shakmaty::Color,
    castling_rights: Bitboard,
}

#[derive(Clone, Copy, Debug)]
struct Board {
    by_role: ByRole<Bitboard>,
    by_color: ByColor<Bitboard>,
    occupied: Bitboard,
}

#[derive(Clone, Debug)]
struct HistoryEntry {
    board: Board,
    turn: shakmaty::Color,
    castling_rights: Bitboard,
}

#[derive(Component, Clone, Debug)]
struct History {
    entries: Vec<HistoryEntry>,
    current: usize,
}

impl Square {
    pub fn new(square: shakmaty::Square) -> Self {
        Self { square }
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
        ))
        .add_event::<DropEvent>()
        .add_event::<SelectEvent>()
        .add_event::<DragEvent>()
        .add_event::<DragEndEvent>()
        .add_event::<EngineEvent>()
        .add_event::<FenEvent>()
        .add_event::<PgnEvent>()
        .add_systems(Startup, startup::startup)
        .add_systems(
            Update,
            (
                update::update,
                clear_indicators::clear_indicators,
                drop::drop.before(drag_end::drag_end),
                select::select,
                drag::drag,
                drag_end::drag_end.after(drop::drop),
                engine::send,
                engine::receive,
                clipboard::clipboard,
                fen::fen,
                pgn::pgn,
                paste::paste,
                back::back,
                forward::forward,
            ),
        )
        .run();
}
