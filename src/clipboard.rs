use bevy::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};

use crate::fen::FenEvent;
use crate::pgn::PgnEvent;

pub(crate) fn clipboard(
    mut evr_fen: EventWriter<FenEvent>,
    mut evr_pgn: EventWriter<PgnEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.all_pressed([KeyCode::ControlLeft, KeyCode::KeyV]) {
        let mut ctx = ClipboardContext::new().unwrap();

        let content = ctx.get_contents().unwrap();

        println!("clipboard: {}", content);

        evr_fen.send(FenEvent::new(content.clone()));
        evr_pgn.send(PgnEvent::new(content));
    }
}
