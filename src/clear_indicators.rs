use crate::{engine::EngineEvent, DropEvent, EngineIndicator, SelectEvent, SelectIndicator};
use bevy::prelude::*;

pub(crate) fn clear_indicators(
    q_select_indicators: Query<(Entity, &SelectIndicator, &Parent)>,
    mut commands: Commands,
    mut ev_select: EventReader<SelectEvent>,
    mut ev_drop: EventReader<DropEvent>,
    mut ev_engine: EventReader<EngineEvent>,
    q_engine_indicators: Query<(Entity, &EngineIndicator, &Parent)>,
) {
    let mut clear = || {
        for indicator in q_select_indicators.iter() {
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
        for engine_move in q_engine_indicators.iter() {
            commands
                .entity(**engine_move.2)
                .remove_children(&[engine_move.0]);
            commands.entity(engine_move.0).despawn();
        }
    }
}
