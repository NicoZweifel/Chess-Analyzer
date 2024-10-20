use crate::{history::History, BoardStartSound, EngineEvent, Game};
use bevy::{audio::Volume, prelude::*};
use shakmaty::fen::Fen;

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
    mut commands: Commands,
    mut q_games: Query<&mut Game>,
    mut q_history: Query<&mut History>,
    mut evr_fen: EventReader<FenEvent>,
    mut evw_engine: EventWriter<EngineEvent>,
    asset_server: Res<AssetServer>,
) {
    for ev in evr_fen.read() {
        if let Ok(fen) = ev.content.parse::<Fen>() {
            let mut game = q_games.get_single_mut().expect("Game not found!");
            let mut history = q_history.get_single_mut().expect("History not found!");

            let pos = fen.into_setup();

            game.setup(pos.clone());

            history.setup(pos);

            commands.spawn((
                AudioBundle {
                    source: asset_server.load("board-start.mp3"),
                    settings: PlaybackSettings {
                        volume: Volume::new(0.5),
                        mode: bevy::audio::PlaybackMode::Despawn,
                        ..default()
                    },
                },
                BoardStartSound,
            ));

            evw_engine.send(EngineEvent);
        }
    }
}
