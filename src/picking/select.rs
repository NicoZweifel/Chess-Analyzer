use crate::{analysis::EngineEvent, ui::SpawnSelectIndicator, Game, Square};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use shakmaty::{Chess, FromSetup, Position};
use std::collections::HashMap;

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct SelectIndicator;

#[derive(Event)]
pub(crate) struct SelectEvent {
    listener: Entity,
}

impl SelectEvent {
    pub fn new(listener: Entity) -> Self {
        Self { listener }
    }
}

impl From<ListenerInput<Pointer<Down>>> for SelectEvent {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        SelectEvent::new(event.listener())
    }
}

pub(crate) fn select(
    mut ev_select: EventReader<SelectEvent>,
    mut ev_engine: EventWriter<EngineEvent>,
    q_games: Query<&Game>,
    mut commands: Commands,
    q_squares: Query<(Entity, &Square)>,
    asset_server: Res<AssetServer>,
) {
    let game = q_games.get_single().expect("Game not found!");

    for event in ev_select.read() {
        ev_engine.send(EngineEvent);

        let square = q_squares.get(event.listener);
        let chess = Chess::from_setup(
            shakmaty::Setup {
                board: shakmaty::Board::from_bitboards(game.board.by_role, game.board.by_color),
                turn: game.turn,
                ep_square: game.ep_square,
                castling_rights: game.castling_rights,
                ..default()
            },
            shakmaty::CastlingMode::Standard,
        )
        .expect("Chess could not load!");

        println!("down {:?} ", square);

        let square_entities: HashMap<shakmaty::Square, Entity> = q_squares
            .into_iter()
            .map(|x| {
                let square = x.1 .0;
                let entity = x.0;
                (square, entity)
            })
            .collect();

        if let Ok((_, square)) = square {
            let moves = chess.legal_moves();

            for m in moves.iter().filter(|&x| x.from() == Some(square.0)) {
                let square = square_entities.get(&m.to()).unwrap();
                commands.spawn_select_indicator(square, &asset_server);
            }
        }
    }
}
