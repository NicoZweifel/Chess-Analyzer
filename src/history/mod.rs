use std::marker::PhantomData;

use crate::{Board, Game};
use bevy::prelude::*;
use shakmaty::Bitboard;

mod back;
mod first;
mod last;
mod next;
mod push_entry_from_pos;
mod setup_from_entry;
mod startup;

pub(crate) use back::*;
pub(crate) use first::*;
pub(crate) use last::*;
pub(crate) use next::*;
pub(crate) use push_entry_from_pos::*;
pub(crate) use setup_from_entry::*;
use startup::startup;

#[derive(Clone, Debug)]
pub(crate) struct HistoryEntry {
    pub(crate) board: Board,
    pub(crate) turn: shakmaty::Color,
    pub(crate) castling_rights: Bitboard,
    pub(crate) ep_square: Option<shakmaty::Square>,
}

#[derive(Component, Clone, Debug)]
pub(crate) struct History {
    pub(crate) entries: Vec<HistoryEntry>,
    pub(crate) current: usize,
}

impl History {
    pub(crate) fn setup(&mut self, setup: shakmaty::Setup) {
        let board = setup.board.clone();
        let (by_role, by_color) = board.into_bitboards();
        let ep_square = setup.ep_square;

        self.entries.clear();
        self.entries.push(HistoryEntry {
            board: Board { by_role, by_color },
            castling_rights: setup.castling_rights,
            turn: setup.turn,
            ep_square,
        });
    }
}

pub struct HistoryPlugin;

impl Plugin for HistoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, (back, next, first, last));
    }
}
