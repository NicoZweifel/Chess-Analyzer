use bevy::prelude::*;

mod clear_indicators;
mod startup;
mod update;

pub(crate) use clear_indicators::*;
pub(crate) use startup::*;
pub(crate) use update::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, (update, clear_indicators));
    }
}
