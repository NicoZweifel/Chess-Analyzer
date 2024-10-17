use crate::Board;
use bevy::prelude::*;
use shakmaty::Bitboard;

mod back;
mod first;
mod forward;
mod last;

pub(crate) use back::back;
pub(crate) use first::first;
pub(crate) use forward::forward;
pub(crate) use last::last;

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
