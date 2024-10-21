use bevy::prelude::*;

mod startup;
mod update;

pub(crate) use startup::*;
pub(crate) use update::*;

#[derive(Event)]
pub(crate) struct SoundEvent {
    pub(crate) sound: String,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>()
            .add_systems(Startup, startup)
            .add_systems(Update, update);
    }
}
