use crate::{drop::DropEvent, engine::EngineEvent, select::SelectEvent, EngineMove, Indicator};
use bevy::prelude::*;

pub(crate) fn clear_indicators(
    q_indicators: Query<(Entity, &Indicator, &Parent)>,
    mut commands: Commands,
    mut ev_select: EventReader<SelectEvent>,
    mut ev_drop: EventReader<DropEvent>,
    mut ev_engine: EventReader<EngineEvent>,
    q_engine_moves: Query<(Entity, &EngineMove, &Parent)>,
) {
    let mut clear = || {
        for indicator in q_indicators.iter() {
            commands
                .entity(**indicator.2)
                .remove_children(&[indicator.0]);
            commands.entity(indicator.0).despawn();
        }
    };

    for _ in ev_select.read() {
        clear();
    }
    for _ in ev_drop.read() {
        clear();
    }
    for _ in ev_engine.read() {
        for engine_move in q_engine_moves.iter() {
            commands
                .entity(**engine_move.2)
                .remove_children(&[engine_move.0]);
            commands.entity(engine_move.0).despawn();
        }
    }
}
