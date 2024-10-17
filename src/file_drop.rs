use crate::{fen::FenEvent, pgn::PgnEvent};
use bevy::prelude::*;
use std::fs::File;
use std::io::prelude::*;

pub(crate) fn file_drop(
    mut evr_dnd: EventReader<FileDragAndDrop>,
    mut evr_fen: EventWriter<FenEvent>,
    mut evr_pgn: EventWriter<PgnEvent>,
) {
    for ev in evr_dnd.read() {
        if let FileDragAndDrop::DroppedFile { window, path_buf } = ev {
            println!(
                "Dropped file with path: {:?}, in window id: {:?}",
                path_buf, window
            );

            if let Ok(mut file) = File::open(path_buf) {
                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {
                    evr_fen.send(FenEvent::new(content.clone()));
                    evr_pgn.send(PgnEvent::new(content));
                }
            }
        }
    }
}
