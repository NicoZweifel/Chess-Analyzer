use bevy::prelude::*;

use super::SoundEvent;

pub(crate) fn startup(mut evr_sounds: EventWriter<SoundEvent>) {
    evr_sounds.send(SoundEvent {
        sound: "board-start.mp3".to_string(),
    });
}
