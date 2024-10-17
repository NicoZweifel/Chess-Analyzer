use std::path::PathBuf;

use bevy::prelude::*;
use shakmaty::{Bitboard, ByColor, ByRole};

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct Piece(pub(crate) shakmaty::Piece);

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct Indicator;

#[derive(Component, Clone, Debug)]
pub(crate) struct Engine(pub(crate) PathBuf);

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct EngineMove;

#[derive(Component, Clone, Debug, Copy)]
pub(crate) struct Square {
    pub(crate) square: shakmaty::Square,
}

#[derive(Component, Clone, Debug)]
pub(crate) struct Game {
    pub(crate) board: Board,
    pub(crate) turn: shakmaty::Color,
    pub(crate) castling_rights: Bitboard,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Board {
    pub(crate) by_role: ByRole<Bitboard>,
    pub(crate) by_color: ByColor<Bitboard>,
}

impl Square {
    pub fn new(square: shakmaty::Square) -> Self {
        Self { square }
    }
}

#[derive(Component)]
pub(crate) struct BoardStartSound;

#[derive(Component)]
pub(crate) struct CaptureSound;

#[derive(Component)]
pub(crate) struct PlacementSound;
