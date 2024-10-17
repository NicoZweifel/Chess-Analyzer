use crate::{
    engine::EngineEvent,
    history::{History, HistoryEntry},
    Board, BoardStartSound, Game,
};
use bevy::{audio::Volume, prelude::*};
use pgn_reader::{BufferedReader, RawHeader, SanPlus, Skip, Visitor};
use shakmaty::{fen::Fen, CastlingMode, Chess, Position};

#[derive(Clone, Debug)]
struct Positions {
    entries: Vec<HistoryEntry>,
    pos: Chess,
}

impl Positions {
    fn new() -> Positions {
        Positions {
            pos: Chess::default(),
            entries: Vec::new(),
        }
    }
}

impl Visitor for Positions {
    type Result = Chess;

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        // Support games from a non-standard starting position.
        if key == b"FEN" {
            let pos = Fen::from_ascii(value.as_bytes())
                .ok()
                .and_then(|f| f.into_position(CastlingMode::Standard).ok());

            if let Some(pos) = pos {
                self.pos = pos;
            }
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn san(&mut self, san_plus: SanPlus) {
        if let Ok(m) = san_plus.san.to_move(&self.pos) {
            self.pos.play_unchecked(&m);

            let board = self.pos.board().clone();
            let (by_role, by_color) = board.into_bitboards();
            let castles = self.pos.castles();
            let ep_square = self.pos.ep_square(shakmaty::EnPassantMode::Legal);
            let entry = HistoryEntry {
                board: Board { by_role, by_color },
                castling_rights: castles.castling_rights(),
                turn: self.pos.turn(),
                ep_square,
            };

            self.entries.push(entry);
        }
    }

    fn end_game(&mut self) -> Self::Result {
        std::mem::take(&mut self.pos)
    }
}

#[derive(Event)]
pub(crate) struct PgnEvent {
    content: String,
}

impl PgnEvent {
    pub(crate) fn new(content: String) -> Self {
        Self { content }
    }
}

pub(crate) fn pgn(
    mut commands: Commands,
    mut q_games: Query<&mut Game>,
    mut q_history: Query<&mut History>,
    mut evw_engine: EventWriter<EngineEvent>,
    mut evr_pgn: EventReader<PgnEvent>,
    asset_server: Res<AssetServer>,
) {
    for ev in evr_pgn.read() {
        let content = ev.content.clone();

        let mut reader = BufferedReader::new_cursor(&content[..]);
        let mut visitor = Positions::new();

        if let Ok(Some(chess)) = reader.read_game(&mut visitor) {
            let mut game = q_games.get_single_mut().expect("Game not found!");
            let mut history = q_history.get_single_mut().expect("History not found!");

            let board = chess.board().clone();
            let (by_role, by_color) = board.into_bitboards();

            game.board = Board { by_role, by_color };
            game.castling_rights = chess.castles().castling_rights();
            game.turn = chess.turn();
            game.ep_square = chess.ep_square(shakmaty::EnPassantMode::Legal);

            history.entries.clear();
            history.entries = visitor.entries.clone();
            let len = visitor.entries.len();
            history.current = if len > 0 { len - 1 } else { 0 };

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
