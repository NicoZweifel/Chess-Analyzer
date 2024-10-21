use super::{fen::FenEvent, pgn::PgnEvent};
use bevy::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};

pub(crate) fn clipboard_paste(
    mut evr_fen: EventWriter<FenEvent>,
    mut evr_pgn: EventWriter<PgnEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_released(KeyCode::KeyV) && keys.all_pressed([KeyCode::ControlLeft]) {
        let mut ctx = ClipboardContext::new().unwrap();

        let content = ctx.get_contents().unwrap();

        println!("clipboard: {}", content);

        evr_fen.send(FenEvent::new(content.clone()));
        evr_pgn.send(PgnEvent::new(content));
    }
}
