use shakmaty::{Chess, Position};

use crate::Board;

use super::{History, HistoryEntry};

pub trait PushEntryFromPos {
    fn push_entry_from_pos(&mut self, pos: Chess);
}

impl PushEntryFromPos for History {
    fn push_entry_from_pos(&mut self, pos: Chess) {
        let board = pos.board().clone();
        let (by_role, by_color) = board.into_bitboards();
        let castles = pos.castles();
        let ep_square = pos.ep_square(shakmaty::EnPassantMode::Legal);

        self.entries.push(HistoryEntry {
            board: Board { by_role, by_color },
            castling_rights: castles.castling_rights(),
            turn: pos.turn(),
            ep_square,
        });

        if self.entries.len() > 1 {
            self.current = self.entries.len() - 1;
        }
    }
}
