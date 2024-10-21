use bevy::prelude::Commands;

use super::History;

pub(crate) fn startup(mut commands: Commands) {
    commands.spawn(History {
        entries: Vec::new().to_vec(),
        current: 0,
    });
}
