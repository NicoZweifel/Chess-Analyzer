use crate::{engine::EngineEvent, Board, Game};
use bevy::prelude::*;
use shakmaty::{Chess, FromSetup, Position};

use super::History;

pub(crate) fn forward(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_games: Query<&mut Game>,
    mut q_history: Query<&mut History>,
    mut ev_engine: EventWriter<EngineEvent>,
) {
    let mut game = q_games.get_single_mut().expect("Game not found!");
    let mut history = q_history.get_single_mut().expect("History not found!");
    if !keys.pressed(KeyCode::ControlLeft)
        && keys.any_just_released([KeyCode::ArrowRight, KeyCode::Enter])
        && history.current < history.entries.len() - 1
    {
        history.current += 1;

        let next = history.entries[history.current].clone();

        let pos = Chess::from_setup(
            shakmaty::Setup {
                board: shakmaty::Board::from_bitboards(next.board.by_role, next.board.by_color),
                turn: next.turn,
                castling_rights: next.castling_rights,
                ..default()
            },
            shakmaty::CastlingMode::Standard,
        )
        .expect("Chess could not load!");

        let board = pos.board().clone();
        let (by_role, by_color) = board.into_bitboards();
        let castles = pos.castles();

        game.board = Board { by_role, by_color };
        game.castling_rights = castles.castling_rights();
        game.turn = pos.turn();

        ev_engine.send(EngineEvent);
    }
}
