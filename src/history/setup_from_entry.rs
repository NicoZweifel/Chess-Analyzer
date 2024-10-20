use bevy::prelude::*;

use crate::Game;

use super::HistoryEntry;

pub trait SetupFromEntry {
    fn setup_history_entry(&mut self, entry: HistoryEntry);
}

impl SetupFromEntry for Game {
    fn setup_history_entry(&mut self, entry: HistoryEntry) {
        self.setup(shakmaty::Setup {
            board: shakmaty::Board::from_bitboards(entry.board.by_role, entry.board.by_color),
            turn: entry.turn,
            castling_rights: entry.castling_rights,
            ep_square: entry.ep_square,
            ..default()
        });
    }
}
