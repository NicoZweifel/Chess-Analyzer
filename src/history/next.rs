use crate::analysis::EngineEvent;
use bevy::prelude::*;

use super::{History, HistoryEntry, SetupFromEntry};

pub(crate) fn next<T: Component + Next, G: Component + SetupFromEntry>(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_games: Query<&mut G>,
    mut q_next: Query<&mut T>,
    mut ev_engine: EventWriter<EngineEvent>,
) {
    let mut game = q_games.single_mut();
    let mut history = q_next.single_mut();

    if !keys.pressed(KeyCode::ControlLeft)
        && keys.any_just_released([KeyCode::ArrowRight, KeyCode::Enter])
    {
        if let Some(next) = history.next() {
            game.setup_history_entry(next);
            ev_engine.send(EngineEvent);
        }
    }
}

pub trait Next {
    fn next(&mut self) -> Option<HistoryEntry>;
}

impl Next for History {
    fn next(&mut self) -> Option<HistoryEntry> {
        if self.current < self.entries.len() - 1 {
            self.current += 1;
            return Some(self.entries[self.current].clone());
        }
        None
    }
}
