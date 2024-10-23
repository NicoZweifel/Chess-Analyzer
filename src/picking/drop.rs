use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{analysis::EngineEvent, history::PushEntryFromPos, Game, History, Piece, Play, Square};

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
    let mut game = q_games.single_mut();
    let mut history = q_history.single_mut();

    for event in ev_drop.read() {
        let mut from = q_squares.get(event.dropped);
        let to = q_squares.get(event.listener);

        if from.is_err() {
            if let Ok(piece) = q_pieces.get(event.dropped) {
                from = q_squares.get(**piece.1)
            }
        }

        if let Ok(from_square) = from {
            if let Ok(to_square) = to {
                if let Ok(c) = game.play(*from_square, *to_square) {
                    history.push_entry_from_pos(c);

                    ev_engine.send(EngineEvent);
                }
            }
        }
    }
}
