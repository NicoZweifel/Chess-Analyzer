use crate::{engine::EngineEvent, Game};
use bevy::prelude::*;

use super::{History, HistoryEntry, SetupFromEntry};

pub(crate) fn back<T: Component + Back, G: Component + SetupFromEntry>(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_games: Query<&mut Game>,
    mut q_back: Query<&mut T>,
    mut ev_engine: EventWriter<EngineEvent>,
) {
    let mut game = q_games.single_mut();
    let mut history = q_back.single_mut();

    if !keys.pressed(KeyCode::ControlLeft)
        && keys.any_just_released([KeyCode::ArrowLeft, KeyCode::Backspace])
    {
        if let Some(previous) = history.back() {
            game.setup_history_entry(previous);
            ev_engine.send(EngineEvent);
        }
    }
}

pub(crate) trait Back {
    fn back(&mut self) -> Option<HistoryEntry>;
}

impl Back for History {
    fn back(&mut self) -> Option<HistoryEntry> {
        if self.current > 0 {
            self.current -= 1;
            return Some(self.entries[self.current].clone());
        }
        None
    }
}
