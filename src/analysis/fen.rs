use crate::{audio::SoundEvent, history::History, Game};
use bevy::prelude::*;
use shakmaty::fen::Fen;

use super::EngineEvent;

#[derive(Event)]
pub(crate) struct FenEvent {
    content: String,
}

impl FenEvent {
    pub(crate) fn new(content: String) -> Self {
        Self { content }
    }
}

pub(crate) fn fen(
    mut q_games: Query<&mut Game>,
    mut q_history: Query<&mut History>,
    mut evr_fen: EventReader<FenEvent>,
    mut evw_engine: EventWriter<EngineEvent>,
    mut evr_sounds: EventWriter<SoundEvent>,
) {
    for ev in evr_fen.read() {
        if let Ok(fen) = ev.content.parse::<Fen>() {
            let mut game = q_games.get_single_mut().expect("Game not found!");
            let mut history = q_history.get_single_mut().expect("History not found!");

            let pos = fen.into_setup();

            game.setup(pos.clone());

            history.setup(pos);

            evr_sounds.send(SoundEvent {
                sound: "board-start.mp3".to_string(),
            });

            evw_engine.send(EngineEvent);
        }
    }
}
