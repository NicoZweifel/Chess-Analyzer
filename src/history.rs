use crate::{engine::EngineEvent, Board, Game};
use bevy::prelude::*;
use shakmaty::{Bitboard, Chess, FromSetup, Position};

#[derive(Clone, Debug)]
pub(crate) struct HistoryEntry {
    pub(crate) board: Board,
    pub(crate) turn: shakmaty::Color,
    pub(crate) castling_rights: Bitboard,
}

#[derive(Component, Clone, Debug)]
pub(crate) struct History {
    pub(crate) entries: Vec<HistoryEntry>,
    pub(crate) current: usize,
}

pub(crate) fn back(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_games: Query<&mut Game>,
    mut q_history: Query<&mut History>,
    mut ev_engine: EventWriter<EngineEvent>,
) {
    let mut game = q_games.get_single_mut().expect("Game not found!");
    let mut history = q_history.get_single_mut().expect("History not found!");
    if !keys.pressed(KeyCode::ControlLeft)
        && keys.any_just_released([KeyCode::ArrowLeft, KeyCode::Backspace])
        && history.current - 1 > 0
    {
        let previous = history.entries[history.current - 1].clone();

        let pos = Chess::from_setup(
            shakmaty::Setup {
                board: shakmaty::Board::from_bitboards(
                    previous.board.by_role,
                    previous.board.by_color,
                ),
                turn: previous.turn,
                castling_rights: previous.castling_rights,
                ..default()
            },
            shakmaty::CastlingMode::Standard,
        )
        .expect("Chess could not load!");

        history.current -= 1;
        let board = pos.board().clone();
        let (by_role, by_color) = board.into_bitboards();
        let castles = pos.castles();

        game.board = Board { by_role, by_color };
        game.castling_rights = castles.castling_rights();
        game.turn = pos.turn();

        ev_engine.send(EngineEvent);
    }
}

pub(crate) fn forward(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_games: Query<&mut Game>,
    mut q_history: Query<&mut History>,
    mut ev_engine: EventWriter<EngineEvent>,
) {
    let mut game = q_games.get_single_mut().expect("Game not found!");
    let mut history = q_history.get_single_mut().expect("History not found!");
    if !keys.pressed(KeyCode::ControlLeft)
        && keys.just_released(KeyCode::ArrowRight)
        && history.current + 1 < history.entries.len()
    {
        let previous = history.entries[history.current - 1].clone();

        let pos = Chess::from_setup(
            shakmaty::Setup {
                board: shakmaty::Board::from_bitboards(
                    previous.board.by_role,
                    previous.board.by_color,
                ),
                turn: previous.turn,
                castling_rights: previous.castling_rights,
                ..default()
            },
            shakmaty::CastlingMode::Standard,
        )
        .expect("Chess could not load!");

        history.current += 1;

        let board = pos.board().clone();
        let (by_role, by_color) = board.into_bitboards();
        let castles = pos.castles();

        game.board = Board { by_role, by_color };
        game.castling_rights = castles.castling_rights();
        game.turn = pos.turn();

        ev_engine.send(EngineEvent);
    }
}

pub(crate) fn first(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_games: Query<&mut Game>,
    mut q_history: Query<&mut History>,
    mut ev_engine: EventWriter<EngineEvent>,
) {
    let mut game = q_games.get_single_mut().expect("Game not found!");
    let mut history = q_history.get_single_mut().expect("History not found!");
    if keys.pressed(KeyCode::ControlLeft)
        && keys.just_released(KeyCode::ArrowLeft)
        && !history.entries.is_empty()
    {
        let previous = history.entries[0].clone();

        let pos = Chess::from_setup(
            shakmaty::Setup {
                board: shakmaty::Board::from_bitboards(
                    previous.board.by_role,
                    previous.board.by_color,
                ),
                turn: previous.turn,
                castling_rights: previous.castling_rights,
                ..default()
            },
            shakmaty::CastlingMode::Standard,
        )
        .expect("Chess could not load!");

        history.current = 0;

        let board = pos.board().clone();
        let (by_role, by_color) = board.into_bitboards();
        let castles = pos.castles();

        game.board = Board { by_role, by_color };
        game.castling_rights = castles.castling_rights();
        game.turn = pos.turn();

        ev_engine.send(EngineEvent);
    }
}

pub(crate) fn last(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_games: Query<&mut Game>,
    mut q_history: Query<&mut History>,
    mut ev_engine: EventWriter<EngineEvent>,
) {
    let mut game = q_games.get_single_mut().expect("Game not found!");
    let mut history = q_history.get_single_mut().expect("History not found!");
    if keys.pressed(KeyCode::ControlLeft)
        && keys.just_released(KeyCode::ArrowRight)
        && !history.entries.is_empty()
    {
        let previous = history.entries[history.entries.len() - 1].clone();

        let pos = Chess::from_setup(
            shakmaty::Setup {
                board: shakmaty::Board::from_bitboards(
                    previous.board.by_role,
                    previous.board.by_color,
                ),
                turn: previous.turn,
                castling_rights: previous.castling_rights,
                ..default()
            },
            shakmaty::CastlingMode::Standard,
        )
        .expect("Chess could not load!");

        history.current = history.entries.len() - 1;

        let board = pos.board().clone();
        let (by_role, by_color) = board.into_bitboards();
        let castles = pos.castles();

        game.board = Board { by_role, by_color };
        game.castling_rights = castles.castling_rights();
        game.turn = pos.turn();

        ev_engine.send(EngineEvent);
    }
}
