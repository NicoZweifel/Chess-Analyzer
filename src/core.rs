use bevy::prelude::*;
use shakmaty::{Bitboard, ByColor, ByRole, CastlingMode, Chess, FromSetup, Position};

#[derive(Component, Clone, Copy, Debug)]
pub struct Piece(pub shakmaty::Piece);

#[derive(Component, Clone, Debug, Copy)]
pub struct Square {
    pub square: shakmaty::Square,
}

#[derive(Component, Clone, Debug)]
pub struct Game {
    pub board: Board,
    pub turn: shakmaty::Color,
    pub castling_rights: Bitboard,
    pub ep_square: Option<shakmaty::Square>,
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub by_role: ByRole<Bitboard>,
    pub by_color: ByColor<Bitboard>,
}

impl Square {
    pub fn new(square: shakmaty::Square) -> Self {
        Self { square }
    }
}

impl Game {
    pub fn default() -> Self {
        let start = Chess::default();
        let board = start.board().clone();
        let (by_role, by_color) = board.into_bitboards();
        Self {
            board: Board { by_role, by_color },
            castling_rights: start.castles().castling_rights(),
            turn: start.turn(),
            ep_square: start.ep_square(shakmaty::EnPassantMode::Legal),
        }
    }

    pub fn setup(&mut self, setup: shakmaty::Setup) {
        self.setup_pos(
            Chess::from_setup(setup, CastlingMode::Standard).expect("Chess could not load!"),
        );
    }

    pub fn setup_pos(&mut self, pos: Chess) {
        let board = pos.board().clone();
        let (by_role, by_color) = board.into_bitboards();
        let castles = pos.castles();

        self.board = Board { by_role, by_color };
        self.castling_rights = castles.castling_rights();
        self.turn = pos.turn();
        self.ep_square = pos.ep_square(shakmaty::EnPassantMode::Legal);
    }
}
