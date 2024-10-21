use bevy::prelude::*;

use super::SoundEvent;

pub(crate) fn update(
    mut commands: Commands,
    mut ev_sound: EventReader<SoundEvent>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_sound.read() {
        commands.spawn((AudioBundle {
            source: asset_server.load(ev.sound.clone()),
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::new(0.5),
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
        },));
    }
}
