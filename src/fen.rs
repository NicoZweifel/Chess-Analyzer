use bevy::prelude::*;
use shakmaty::fen::Fen;

use crate::{Board, Game, History, HistoryEntry};

#[derive(Event)]
pub(crate) struct FenEvent {
    content: String,
}

impl FenEvent {
    pub(crate) fn new(content: String) -> Self {
        Self { content }
    }
}

pub(crate) fn fen(
    mut q_games: Query<&mut Game>,
    mut q_history: Query<&mut History>,
    mut evr_fen: EventReader<FenEvent>,
) {
    for ev in evr_fen.read() {
        if let Ok(fen) = ev.content.parse::<Fen>() {
            let mut game = q_games.get_single_mut().expect("Game not found!");
            let mut history = q_history.get_single_mut().expect("History not found!");

            let chess = fen.into_setup();
            let board = chess.board.clone();
            let (by_role, by_color) = board.into_bitboards();

            game.board = Board { by_role, by_color };
            game.castling_rights = chess.castling_rights;
            game.turn = chess.turn;

            history.entries.clear();
            history.entries.push(HistoryEntry {
                board: Board { by_role, by_color },
                castling_rights: chess.castling_rights,
                turn: chess.turn,
            });
        }
    }
}
