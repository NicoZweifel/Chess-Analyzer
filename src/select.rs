use std::collections::HashMap;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use shakmaty::{Chess, FromSetup, Position};

use crate::{engine::EngineEvent, Game, Indicator, Square};

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
    let game = q_games.iter().next().expect("Game not found!");

    for event in ev_select.read() {
        ev_engine.send(EngineEvent);

        let square = q_squares.get(event.listener);
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

        println!("down {:?} ", square);

        let square_entities: HashMap<shakmaty::Square, Entity> = q_squares
            .into_iter()
            .map(|x| {
                let square = x.1.square;
                let entity = x.0;
                (square, entity)
            })
            .collect();

        if let Ok((_, square)) = square {
            let moves = chess.legal_moves();

            for m in moves.iter().filter(|&x| x.from() == Some(square.square)) {
                let square = square_entities.get(&m.to()).unwrap();
                let texture: Handle<Image> = asset_server.load("Name=Off, Hint=On.png");

                let child = commands
                    .spawn((
                        Indicator,
                        SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(0.0, 0.0, 0.3),
                            ..default()
                        },
                    ))
                    .id();

                commands.entity(*square).push_children(&[child]);
            }
        }
    }
}
