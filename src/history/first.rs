use crate::analysis::EngineEvent;
use bevy::prelude::*;

use super::{History, HistoryEntry, SetupFromEntry};

pub(crate) fn first<T: Component + First, G: Component + SetupFromEntry>(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_games: Query<&mut G>,
    mut q_first: Query<&mut T>,
    mut ev_engine: EventWriter<EngineEvent>,
) {
    let mut game = q_games.single_mut();
    let mut history = q_first.single_mut();

    if keys.pressed(KeyCode::ControlLeft) && keys.just_released(KeyCode::ArrowLeft) {
        if let Some(first) = history.first() {
            game.setup_history_entry(first);
            ev_engine.send(EngineEvent);
        }
    }
}

pub trait First {
    fn first(&mut self) -> Option<HistoryEntry>;
}

impl First for History {
    fn first(&mut self) -> Option<HistoryEntry> {
        if !self.entries.is_empty() {
            self.current = 0;
            return Some(self.entries[self.current].clone());
        }
        None
    }
}
