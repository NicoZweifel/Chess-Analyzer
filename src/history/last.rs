use crate::analysis::EngineEvent;
use bevy::prelude::*;

use super::{History, HistoryEntry, SetupFromEntry};

pub(crate) fn last<T: Component + Last, G: Component + SetupFromEntry>(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_games: Query<&mut G>,
    mut q_last: Query<&mut T>,
    mut ev_engine: EventWriter<EngineEvent>,
) {
    let mut game = q_games.single_mut();
    let mut history = q_last.single_mut();

    if keys.pressed(KeyCode::ControlLeft) && keys.just_released(KeyCode::ArrowRight) {
        if let Some(last) = history.last() {
            game.setup_history_entry(last);
            ev_engine.send(EngineEvent);
        }
    }
}

pub trait Last {
    fn last(&mut self) -> Option<HistoryEntry>;
}

impl Last for History {
    fn last(&mut self) -> Option<HistoryEntry> {
        if !self.entries.is_empty() {
            self.current = self.entries.len() - 1;
            return Some(self.entries[self.current].clone());
        }
        None
    }
}
