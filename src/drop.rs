use crate::{
    engine::EngineEvent,
    history::{History, HistoryEntry},
    Board, Game, Piece, Square,
};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use shakmaty::{Chess, FromSetup, Position};

#[derive(Event)]
pub(crate) struct DropEvent {
    listener: Entity,
    dropped: Entity,
}

impl DropEvent {
    pub fn new(listener: Entity, dropped: Entity) -> Self {
        Self { listener, dropped }
    }
}

impl From<ListenerInput<Pointer<Drop>>> for DropEvent {
    fn from(event: ListenerInput<Pointer<Drop>>) -> Self {
        DropEvent::new(event.listener(), event.dropped)
    }
}

pub(crate) fn drop(
    mut ev_drop: EventReader<DropEvent>,
    mut q_games: Query<&mut Game>,
    mut q_history: Query<&mut History>,
    mut ev_engine: EventWriter<EngineEvent>,
    q_squares: Query<&Square>,
    q_pieces: Query<(&Piece, &Parent)>,
) {
    let mut game = q_games.get_single_mut().expect("Game not found!");
    let mut history = q_history.get_single_mut().expect("History not found!");

    for event in ev_drop.read() {
        let mut from = q_squares.get(event.dropped);
        let to = q_squares.get(event.listener);

        if from.is_err() {
            if let Ok(piece) = q_pieces.get(event.dropped) {
                from = q_squares.get(**piece.1)
            }
        }

        let chess = Chess::from_setup(
            shakmaty::Setup {
                board: shakmaty::Board::from_bitboards(game.board.by_role, game.board.by_color),
                turn: game.turn,
                castling_rights: game.castling_rights,
                ..default()
            },
            shakmaty::CastlingMode::Standard,
        )
        .expect("Chess could not load!");

        println!("move \n {:?} \n {:?} ", from, to);
        if let Ok(from_square) = from {
            if let Ok(to_square) = to {
                let moves = chess.legal_moves();
                let legal_move = moves
                    .iter()
                    .find(|&x| x.from() == Some(from_square.square) && x.to() == to_square.square);

                println!("move {:?} ", legal_move);
                if let Some(m) = legal_move {
                    match chess.play(m) {
                        Ok(c) => {
                            println!("played {:?} ", legal_move);
                            let board = c.board().clone();
                            let (by_role, by_color) = board.into_bitboards();
                            let castles = c.castles();

                            game.board = Board { by_role, by_color };
                            game.castling_rights = castles.castling_rights();
                            game.turn = c.turn();

                            history.entries.push(HistoryEntry {
                                board: Board { by_role, by_color },
                                castling_rights: castles.castling_rights(),
                                turn: c.turn(),
                            });

                            if history.entries.len() > 1 {
                                history.current += 1;
                            }

                            ev_engine.send(EngineEvent);
                        }
                        Err(_) => todo!(),
                    };
                }
            }
        }
    }
}
