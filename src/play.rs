use crate::{Game, Square};
use bevy::prelude::*;
use shakmaty::{Chess, FromSetup, Position};

pub trait Play {
    fn play(&mut self, from_square: Square, to_square: Square) -> Result<Chess, ()>;
}

impl Play for Game {
    fn play(&mut self, from_square: Square, to_square: Square) -> Result<Chess, ()> {
        let chess = Chess::from_setup(
            shakmaty::Setup {
                board: shakmaty::Board::from_bitboards(self.board.by_role, self.board.by_color),
                turn: self.turn,
                ep_square: self.ep_square,
                castling_rights: self.castling_rights,
                ..default()
            },
            shakmaty::CastlingMode::Standard,
        )
        .expect("Chess could not load!");

        let moves = chess.legal_moves();
        let legal_move = moves
            .iter()
            .find(|&x| x.from() == Some(from_square.0) && x.to() == to_square.0);

        if let Some(m) = legal_move {
            match chess.play(m) {
                Ok(c) => return Ok(self.setup_pos(c)),
                Err(_) => return Err(()),
            }
        }
        Err(())
    }
}
